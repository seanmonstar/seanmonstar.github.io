---
layout: post
title: 'reqwest retries'
date: '2025-07-15T10:39:00'
tags:
- rust
- http
- reqwest
- tower
- retries
- open-source
- programming
---
One part of working on a more [modular `reqwest`][modular] was adding support for retries.

Intermittent failures are a fact of life, and a simple retry could have made things so much better. It's a somewhat frequent feature request for `reqwest`. It's also a very common pattern to implement manually, and far too easy to do incorrectly. Since `reqwest`'s internals are shifting more and more to a stack of services, it felt like an ideal time to explore this.

While skeleton pieces do exist in `tower::retry`, they require Expert Mode. Doing things right is tricky. We can do better.

### Policy composition

A retry policy is several things at once. But each of those things often is only one of a few choices. So what if we made it easier to compose your choices? That is, we can provide the common choices, and a builder to combine them.

#### Budgets by default

Retry counts alone are insufficient, but absurdly common. Please don't do this. We can educate and document and warn and wave our hands, but we can only plead so much.

Budgets will be default on. Sure, there's an escape hatch if you absolutely must. But this way we can help you fall into the pit of success.

And to help users better understand what the budget results in, configuration looks like `.max_extra_load(0.3)`, instead of implementation parameters of the windowed bucket.

#### Scoped retries

Budgets are basically a must. But budgets also don't make sense across different targets. Just because requests to `frobnica.te` start failing doesn't mean retries should also be disabled to `storali.ze`. This dichotomy perturbed me quite a bit, actually, and so I put off trying to solve the whole thing for a few months. But after letting my mind rest, looking at the exact same problem revealed an obvious solution: scopes.[^scopes]

I believe the most likely use of scopes is based on hostname. So that's the easiest constructor to start a builder: `retry::for_host(host)`. But it's actually a sealed trait, to allow continued exploration. Maybe a closure is needed by some people? We'll find out!

#### Cloning requests

In many languages, this isn't even a consideration. You can just share a pointer, and _wave hands_ mostly reuse it.

In Rust, with ownership, we need to keep a clone of the request for a potential retry.

At the `hyper` level, we don't know enough about the request body to automatically clone it. At `reqwest`'s level, we _do_ know enough: we can absolutely clone when it's a simple non-streaming body. (Even possibly more, discussed below.)


#### Classifiers

You might have a very custom combination of routes and responses. Or an API where every idempotent request has standard status codes. Or you might need a deeper understanding of specific connection or protocol errors.

So to start, there's a simpler `classify_fn` option, where you provide a closure to determine whether to retry or not.

```rust
.classify_fn(|req_rep| {
    match (req_rep.method(), req_rep.status()) {
        (&GET, Some(SERVICE_UNAVAILABLE)) => req_rep.retryable(),
        // or check req_rep.error()?
        _ => req_rep.success(),
    }
})
```

Another possible avenue to explore is the composition of classifiers. For example, what if you combined something like `protocol_nacks.or(idempotent.and(service_unavailable))`? Is this a good idea? Would people want this? If you want to _use_ something like that, file an issue or PR.

#### Probable enhancements

Besides the above, there's a few other things we could add to `reqwest::retry`:

- **Backoffs:** They're pretty easy to add, though the benefit they bring is only really noticeable in specific situations. This is why they aren't included yet or would not even be enabled by default.
- **Replayable Body:** Retrying needs to clone the request, and a streaming body is not freely able to be cloned and restarted. But it's desirable by users, so we could make it an easy configurable option, such as `.max_replay_body_bytes()`. We could likely learn a bit from [how Linkerd implemented it](https://linkerd.io/2021/10/26/how-linkerd-retries-http-requests-with-bodies/).
- **Per-Request Retries:** More and more, users ask for configuration for a specific request that differs from the `Client`. We've been working on [general per-request config system](https://github.com/seanmonstar/reqwest/issues/2641).

### Exploring and upstreaming

We're exploring these ideas in reqwest, with the goal of upstreaming the more generic ideas to `tower` or `tower-http`.

Maybe that's a builder:

```rust
tower::retry::builder()
    .clone_reqs(|req| Some(req.clone()))
    .matcher(|req, res| res.status() == RETRY_AFTER)
    .max_extra_load(0.3)
    .build();
```

Or maybe something closer to middleware, a trait to compose pieces, with `and()` or `.or()`:

```rust
tower_http::retry::clone_replayable(8192)
    .and(idempotent)
    .and(server_error)
    .or(protocol_nack)
    .and(budget(0.3))
```

Maybe a combination! Who knows?

### Check it out

There's a [pull request][pr] implementing some of these ideas, and your feedback and usage will help us to keep refining it.

The two-pronged goal is to allow you to easily get retries in reqwest, and to also be able to get that easier configuration in _any_ tower stack.

[Give a whirl!][pr]

[^scopes]: Scoping budgets wouldn't be something to care about if the client were already built for a specific endpoint. That's a common way to build a stack of tower layers. But reqwest is open-ended, you can send requests to anywhere with an address. So, scopes felt required.

[modular]: https://seanmonstar.com/blog/modular-reqwest/
[pr]: https://github.com/seanmonstar/reqwest/pull/2763
