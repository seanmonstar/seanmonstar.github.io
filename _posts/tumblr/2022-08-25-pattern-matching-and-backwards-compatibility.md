---
layout: post
title: Pattern Matching and Backwards Compatibility
date: '2022-08-25T12:17:10-04:00'
tags:
- rust
- rust-lang
- best-practices
- bestof
- programming
tumblr_url: https://seanmonstar.com/post/693574545047683072/pattern-matching-and-backwards-compatibility
---
Pattern matching in Rust is, the first time it clicks, something magical.

When you define a domain subject with an `enum`, and then `match` on the various details, you can express logic in a very natural way, and reduce errors about forgetting to handle all the cases. It’s exceptional when the `enum` definition and the `match` statements are in the same crates.

It’s a bit trickier when a library wants to export variants to allow easy matching for users of that library. There are API compatibility concerns, and while some have elegant solutions, others… don’t.

This write-up shows several instances of this that I’ve learned from a library author’s point of view.

1. Non-exhaustive enums
2. Translating typical Error inheritance
3. Enums with an “open-ended” variant

### Non-exhaustive

The various versions of HTTP are well-defined, and are not variant in the same way a library version might be. You don’t have a whole bunch of unknown HTTP version identifiers that you need to parse. So representing it as an enum is a pretty natural fit. You might start with something like this:

    enum Version {
        Http10,
        Http11,
        Http2,
    }

A user might decide they need to do something different depending on the version of a request received:

    match req.version() {
        Version::Http10 => println!("1.0"),
        Version::Http11 => println!("1.1"),
        Version::Http2 => println!("2"),
    }

That feels quite nice! But then as a library, you realize you need to add a new variant to support HTTP/3. By doing that, you’ll break compilation for the user, since they are trying to exhaustively match on a pattern which is no longer exhaustive. Rust now has an attribute, `#[non_exhaustive]`, which you can place on a definition to protect your users from these changes.[^1]

    #[non_exhaustive]
    enum Version {
        Http10,
        Http11,
        Http2,
    }

The user is now _required_ to include a catch-all pattern, signaling that more variants may appear in the future.

    match req.version() {
        Version::Http10 => println!("1.0"),
        Version::Http11 => println!("1.1"),
        Version::Http2 => println!("2"),
        other => println!("other? {:?}", other),
    }

With that in place, adding `Version::Http3` won’t break your user. Compatibility!

#### Exhaustively non-exhaustive

    #[non_exhaustive]
    enum Version {
        Http10,
        Http11,
        Http2,
    }

The `non_exhaustive` attribute means a user must include a catch-all arm, which means we can add new variants and not _break_ their code.

However, what if the user really wants to know if they handled all possible variants, perhaps after an upgrade? The catch-all prevents them from knowing.

There is a currently [unstable lint](https://github.com/rust-lang/rust/issues/89554), that is `allow` by default, but when developing a user can opt-in to being warned about new variants.

    match req.version() {
        Version::Http10 => println!("1.0"),
        Version::Http11 => println!("1.1"),
        #[warn(non_exhaustive_omitted_patterns)]
        _ => unknown(),
    }

With the current `enum`, this will warn the user they didn’t match on `Version::Http2`. And when the library adds a `Version::Http3` variant, the user will notice a new warning when they upgrade.

### Error types

It’s very common in Rust for errors to be represented by an `enum`. The perceived benefit is that a consumer can cleanly match on different error cases, and if desired, even ensure you had handled _every_ possible error case. While I’m not here to argue the premise of whether you _should_ handle every single case, a problem I’ve [struggled with](https://github.com/hyperium/hyper/issues/1131#issuecomment-362379005) related to this approach is probably better to explain first with a different programming language.

Let’s say we have some client in Python, and we want to specifically retry “connect” errors. We might write that like so:

    try:
        resp = client.get(url)
    except error.Connect:
        retry()
    # otherwise, keep raising

Later on, the client library decides it wants to allow being more specific about the exact error kind that occured while connecting. The library could do that by declaring some new error types that inherit from the `Connect` error.

    class Resolve(error.Connect):
        pass
    class Handshake(error.Connect):
        pass

The user’s original code will continue to work the same. Backwards compatibility has been kept. But a user can also update their code to be more specific, if they so desire:

    try:
        resp = client.get(url)
    except error.Resolve:
        raise # don't retry a bad hostname
    except error.Connect:
        retry()
    # otherwise, keep raising

Back in Rust, this is _harder_ to do. The simplest solution is just not to expose an `enum`, but an opaque struct with methods, like `is_connect()`, which will continue to be `true` if you later add `is_resolve()`. But, we’re talking about pattern matching. Could we make this sort of flexibility work?

We _could_. Probably the better question is _should we?_ But let’s peek first, anyways.

#### Nested Non-exhaustiveness

Let’s see what the user could try to write first, to see the goal:

    match client.get(url) {
        Error::Connect(_) => {
            // retry
        },
        e => return Err(e),
    }

The key here is that the `Connect` variant must contain a field. That way, if the library adds sub-variants, it doesn’t break the code at compile-time or at runtime. It’s also important that when there aren’t any sub-variants, the pattern should still be non-exhaustive.

The library could provide its variants like this:

    #[non_exhaustive]
    pub enum Error {
        Connect(Connect),
        // other top-level errors
    }
    
    #[non_exhaustive]
    pub enum Connect {
        Resolve(unnameable::Resolve),
        Handshake(unnameable::Handshake),
    }
    
    mod unnameable {
        pub struct Resolve(());
        pub struct Handshake(());
    }

The `Connect` sub-variants contain publicly unnameable typed fields, to prevent the user from being too exhaustive. This prevents a pattern such as `Error::Connect(Connect::Resolve(Resolve(_)))`, which would break when `Resolve` became an `enum`. We can’t reach for an empty `enum Resolve {}`, since then the type would be uninhabitable, and we could never _construct_ such an error value.

An upgrading user doesn’t break, but they can also be more specific if they choose:

    match err {
        e @ Error::Connect(Connect::Resolve(_)) => {
            return Err(e); // don't retry
        }
        Error::Connect(_) => {
            // retry
        }
    }

If later on, we want to add some `Resolve` variants, we can promote it to an enum without breakage:

    #[non_exhaustive]
    pub enum Resolve {
        NoRecordsFound(unnameable::NoRecordsFound),
        Io(unnameable::Io),
    }

The question of _should we_ still lingers, though. I can’t say what the right answer is. I leave this here hoping someone will explore the idea more thoroughly, and tell us how great or horrid it is.[^2]

### Open-ended variants

Some other domains _appear_ to lend themselves to being represented by an `enum`, but likely _shouldn’t_ be. These are things where there are a set of commonly known types, but with an additional variant that is purposefully open-ended. Sometimes it’s setup that way to allow for some official registry of constants, so a specification doesn’t need to know all of them at publication time. Others just define the rules of a type, define the most common, and then also include an “extension” variant.

Defining these publicly as a Rust `enum` is an API compatibility hazard. Consider the example of HTTP status codes. Say we defined them like this:

    #[non_exhaustive]
    pub enum Status {
        Ok,
        NotFound,
        InternalServerError,
        Unregistered(u16),
    }

We have an `Unregistered(u16)` variant because the specification allows for new status codes to defined over time, and even unregistered custom codes are valid. But what if a user starts matching on a status to use a newly proposed code:

    match code {
        Status::Unregistered(103) => println!("early hints!"),
        _ => (),
    }

If the library later on decides to add a variant for the new status code, even though the enum was marked as `non_exhaustive`, it will be a form of breaking change. It won’t be a _compile-time_ break. Instead, it will only happen at runtime. The code, as a `Status::EarlyHints`, will no longer match the `Status::Unregistered(103)` pattern.

#### Opaque types with constants

We solved this in [`http`](https://crates.io/crates/http) by making `StatusCode` an opaque struct, and defined all the “variants” as constants.

    pub struct StatusCode(u16);
    
    impl StatusCode {
        pub const OK: StatusCode = StatusCode(200);
        pub const NOT_FOUND: StatusCode = StatusCode(404);
        // etc ...
    }

Users can still match on the common “variants”, but specifically _can’t_ match an exposed variant name for unregistered codes. They can only match them with a catch-all pattern:

    match code {
        StatusCode::OK => println!("ok!"),
        other => {
            if other == 103 {
                println!("early hints!");
            }
        }
    }

Even when a new status code constant is added, that existing code will continue to work. It does make it harder to _exhaustively_ match on all possible status codes, but there are enough of them that the likelihood of anyone wanting to do that is very low.

Turns out, this works well when matching for equality![^3] `http::Method` uses this too. This lesson also inspired a lot of changes in the [`headers`](https://crates.io/crates/headers) crate.



[^1]: We made the `http::Version` type before `#[non_exhaustive]` existed, using a similar technique to `StatusCode`. We could switch it, but seeing them as constants feels _right_.

[^2]: Probably somewhere in the middle, as is the usual when comparing trade-offs.

[^3]: There’s another case regarding open-ended variants, but I think it’s big enough to be a separate post.

