---
layout: post
title: 'This Month in hyper: November 2022'
date: '2022-12-02T10:50:45-05:00'
tags:
- hyper
- rust
- rust-lang
- monthly
- http3
tumblr_url: https://seanmonstar.com/post/702541983003148288/this-month-in-hyper-november-2022
---
The northern hemisphere starts to cool, the trees shed their leaves to conserve energy, cultured fans watch the quadrennial football tournament, and some magnificient contributors stay warm by continuing to make [hyper](https://hyper.rs) ever better!

### Releases

#### hyper v0.14.23

@jfourie1 found and fixed a nasty bug in hyper’s HTTP/2 client dispatcher, which could result in a stalled connection under high concurrency.

#### reqwest v0.11.13

The headline addition is a `ClientBuilder::dns_resolver()`, which now allows users to implement completely custom DNS resolvers for reqwest to use.

### hyper 1.0

With the release of rc1 last month, I wrote about how we’re now in the [hyper polish period](https://seanmonstar.com/blog/2022-11-15-hyper-polish-period/) 💅.

[@programatik29 volunteered](https://github.com/hyperium/hyper/pull/3059) to co-lead the [util area](https://github.com/orgs/hyperium/projects/1/views/7). After many contributions to get us to rc1, [@oddgrd joined us](https://github.com/hyperium/hyper/pull/3065) to lead the [docs area](https://github.com/orgs/hyperium/projects/1/views/8). @vi noticed that the HTTP/2 client `SendRequest` should implement `Clone`.

We still eagerly welcome you trying out the release candidate and providing us feedback. It’s the _most important_ part of this period. Or join us in one of the four polish areas (or help lead one)! You can also come [chat](https://discord.gg/kkwpueZ) with us about anything.

### hyper in curl

I fixed curl’s `c-hyper.c` to [classify headers from CONNECT and 1xx responses as such](https://github.com/curl/curl/pull/9947), making two more [unit tests pass](https://github.com/orgs/hyperium/projects/2/views/1). I streamed the process, in case you’d find it helpful to watch someone who mainly writes Rust flounder around debugging and fixing C. [You can try](https://seanmonstar.com/blog/2022-03-16-help-stabilize-hyper-in-curl/) it too!

### HTTP/3

We’re working on HTTP/3 in a separate crate, [h3](https://github.com/hyperium/h3), with the goal of fitting it into [hyper](https://hyper.rs).

@eagr made it so `recv_trailers` doesn’t require holding onto the `SendStream` side. @g2p cleaned up a huge swath of clippy errors.

@g2p and @Ruben2424 joined as `h3` collaborators, thanks to their excellent and continued work!

### Tower

[Tower](https://github.com/tower-rs/tower) (and `tower-http`) are a protocol-agnostic RPC framework with middleware, and they combine nicely with hyper.

@alexrudy added a new `BoxCloneServiceLayer`, and @leoyvens fixed a couple bugs in `CallAll`. @davidpdrsn made `BoxService` implement `Sync`.

Oh, and while not Tower, there was a big new [Axum v0.6 release](https://tokio.rs/blog/2022-11-25-announcing-axum-0-6-0)!

