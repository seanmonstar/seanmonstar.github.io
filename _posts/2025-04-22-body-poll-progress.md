---
layout: post
title: 'Body::poll_progress'
date: '2025-04-22T10:01:00'
tags:
- rust
- http
- hyper
- open-source
- programming
---
This describes a proposal for a cancelation problem with hyper's request and response bodies. [hyper][] is an HTTP library for the Rust language.

### Background: what is the `Body` trait?

The `Body` trait used by [hyper][] is meant to represent a potentially streaming (asynchronous) body of a request or response. It sorta looks like `Stream` or `AsyncIterator`.

The biggest reason for a different trait was because we needed a stable trait for [hyper v1][], and `Stream`/`AsyncIterator` were not going to be (and still aren't) stable. With it being a separate trait, though, we also added some HTTP-body-specific methods to it.

Its similarity to `Stream`/`AsyncIterator` means we run into a problem with forwarding into a sink. But that we own the trait also lends to a decent solution, which this post outlines.

First though, piping woes.

### Problem: backpressured cancelation
 
Piping stream-like things into sink-like destinations feels natural, and looks quite simple. Elegant even. They can be piped together, and backpressure occurs naturally. However, they lack a mechanism to completely propagate cancelation. More specifically, cancelation while backpressure is currently applied.

Consider an example:

```rust
while let Some(frame) = body.frame().await? {
    // what if body (^) cancels while we wait here?
    dst.send(frame).await?;
}
```

This simple loop is piping a `Body` into some sort of sink. The way most streams and channels work, this mostly propagates backpressure. As long as the `dst` is not able to send a frame, we don't poll for another one, and whatever is the source of the body will back up.

However, it has a flaw. Those familiar with writing proxies may notice it immediately. The destination might not have space, and so we'll wait for it to get more. But the body could give up during that time. Since the task is only waiting on when the `dst` is ready, it could wait a significant amount of time and never notice that the body (source) has canceled.

#### Why not just timeouts?

One initial question was "why not just timeouts". Like, why not just add `body_write_timeout()` or something to hyper's connection builders. Fair. That would work in some cases, for sure.

The thing is, sometimes timeouts are inappropriate.

For example, a transparent proxy may not want to force timeouts where they didn't exist before. They're willing to wait nearly forever. They just want to cancel the forwarding if the sender gives up.

Additionally, a chain of potential timeouts means that cancelation propagation can be delayed longer and longer, as each hop in the chain has to wait its own timeout.

#### More generally, yet not

This issue can also exist for `Stream`/`AsyncIterator`. A [`poll_progress` idea was outlined by withoutboats](https://without.boats/blog/poll-progress/), though it was solving a different problem.

I do think there's some crossover that may affect each other. But it's also worth considering separately.

I'm not talking about a concept that some `for await` syntax could magically care for. I don't want that. Like at all. In the above example, it'd be impossible for a compiler to determine which action I want to take. Maybe the source ended cleanly. I don't want that to cancel sending the last item.

Also, and this is a biggie: `Body` will need to solve this problem quicker.

### Solution: `Body::poll_progress()`

We [propose][pr] to add a method to the `Body` trait:

```rust
pub trait Body {
    // existing methods ...
    
    fn poll_progress(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
```

The purpose of the method is allow for it to make any "progress" that might result in cancelation, while not producing another frame. The details:

- It's a `poll_*` function, so it allows a body to poll something else async, like a timeout, or perhaps some shutdown channel.
- Returning `Ready(Ok(()))` immediately is fine, it simply means there's _nothing_ that the body would do before another frame could be made available.
- However, returning an `Err` would indicate that the body is canceled, and a piping task can interpret that to shutdown the forwarding operation.

We could provide some utilities to aid use of this function, such as `http_body_util::BodyExt::progress()`.

Consider an example, something that could be even be used in `hyper` internals, but could also occur elsewhere:

```rust
loop {
    // produce a frame
    let frame = select! {
        Some(frame) = body.frame() => frame,
        dst.hup() => {
            // unexpected EOF
            return;
        }
    };

    // send it
    select! {
        dst.write(frame) => {
            // continue
        },
        Err(e) = body.progress() => {
            dst.abort();
            return;
        }
    }
}
```

#### Alternatives: not `poll_closed`

An alternative we considered was making this method be a `poll_closed`. It had the welcome effect of meaning we could await `body.closed()`, and that is fairly self-documenting. However, it ran into several problems.

Being an addition to an existing trait, it needs a default implementation. But a default for `poll_closed` would likely end up being confusing. If it returns closed by default, then existing `impl Body`s will suddenly start aborting early. If it returns not-closed, then any naive task that might await `body.closed()` would hang forever.

The return value also felt confusing however we put it. Does `closed` return a `Result`? Both `Ok` and `Err` would still mean the body had closed.

### Request for Comments

I wrote this up because it felt like a big enough change to a fundamental mechanism in the ecosystem that it'd could benefit from more eyes and comments. It kinda-sorta looks like `AsyncIterator::poll_progress`, but it's also not. Still, are there things we should prepare for? Would wg-async have thoughts? Others who are using hyper deeply?

Could it be simpler? Is it flexible enough?[^redesign]

Comments on the [pull request are most welcome][pr]![^thanks]

[^thanks]: Thanks to Steven Fackler and Oliver Gould for helping iterate on this design.

[^redesign]: Calls for completely changing hyper's design to not need such a function aren't helpful. hyper is stable at v1.

[hyper]: https://hyper.rs
[hyper v1]: https://seanmonstar.com/blog/hyper-v1/
[pr]: https://github.com/hyperium/http-body/pull/90
