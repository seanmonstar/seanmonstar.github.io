---
layout: post
title: hyper-ish 2021 in review
date: '2022-01-04T13:19:28-05:00'
tags:
- rust
- rust-lang
- hyper
- http
- curl
- tower
- tokio
- console
- aws
- reinvent
tumblr_url: https://seanmonstar.com/post/672473147126300672/hyper-ish-2021-in-review
---
2021 is done. Finally. I thought it’d be fun to look back and highlight some of things that have happened that are related to [hyper](https://hyper.rs).<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>

### re:Invent

[![reinvent video](https://i.imgur.com/dKt54sN.png)](https://youtu.be/MZyleK8elPk)

In January 2021, Carl Lerche and I presented a talk for re:Invent, [Next Gen networking infrastructure with Rust and Tokio](https://youtu.be/MZyleK8elPk). It gives a good overview to someone who isn’t already using Rust.

### hyper

The very first commit of the year to [hyper](https://hyper.rs) was the [initial C API](https://github.com/hyperium/hyper/pull/2278), but I’ll save more about that for its own section. The number of unique contributors grew 10% from 2020, with a total of 47 humans helping fix code, examples, and documentation in 2021. We introduced two [new core members](https://hyper.rs/blog/2021/05/06/welcome-anthony-ramine/) working on hyper-related projects.

All those contributors helped with 14 hyper releases. Those releases includes HTTP/2 `CONNECT` support. The client can now understand HTTP/0.9 responses. There were performance improvements adjusting HTTP/2 adaptive window pings, and the memory used with the flatten write buffer strategy.

I spoke on the [Rustacean Station podcast](https://rustacean-station.org/episode/045-sean-arthur/) about the history of hyper, async Rust, Rust in C, the future of hyper, [and more](https://seanmonstar.com/blog/podcast-hyper-on-the-rustacean-station/).

I’ve been working on a plan and vision for hyper, and that should be finalized in January 2022. **1.0 is coming!**

### hyper in curl

We started working on a C API for hyper in 2020, to [allow curl to use hyper](https://daniel.haxx.se/blog/2020/10/09/rust-in-curl-with-hyper/) as an HTTP backend. At the start of 2021, the core of that API was merged. We hosted a [webinar](https://youtu.be/okGUxW_i9yk) explaining why and how it was working, and I wrote a bit more about the effort on the [AWS Open Source blog](https://aws.amazon.com/blogs/opensource/how-using-hyper-in-curl-can-help-make-the-internet-safer/). As I said then:

> Considering how much curl is used, this was an opportunity to make the **internet** safer.

We’ve since added a few more features and knobs to the C API of hyper. The [documentation](https://docs.rs/hyper/latest/hyper/ffi/) is now rendered. And we got an [RFC for `cargo rustc --crate-type` approved](https://github.com/rust-lang/rfcs/pull/3180), and there’s now nightly support. This will allow turning on the C library at command-line time, instead of being always-on in the `Cargo.toml`.

In curl, the number of test cases left to port keeps shrinking. A couple need some future work in hyper, but most just need some help updating curl’s internals. Whittling this list down to zero is an important step in getting curl built with hyper into people’s hands. It’s so close!

### HTTP/3

Also as mentioned [before](https://seanmonstar.com/blog/hyper-v014/), we’ve been working on an HTTP/3 library in the similar vein as `h2`, which is generic over any QUIC implementation.

At the end of 2020, we had a working-ish client example. Thanks to the hard work of [Jean-Christophe Begue](https://github.com/stammw), we now have a working server example. The client example fully works. The project updated to Tokio 1.0, and quinn 0.8: working with “final” version of the h3 specification. It gained proper control stream support, and reduced copies when reading from the transport.

Look forward to an initial release in early 2022, and exploratory support in [reqwest](https://crates.io/crates/reqwest).

### Tower

Tower is a library of modular and reusable components for building robust networking clients and servers. At its core is the `Service` trait, which just an abstraction of an async function turning a request into a response. You can describe a _lot_ of things with this trait. And it’s been our goal for years that the ecosystem can swap out HTTP implementations by just being generic over any HTTP `Service`.

Tower is more than just that one trait, however. With that basic abstraction in place, Tower includes a lot of middleware that is useful in all kinds of networking, regardless of protocol, whether server or client.

[David Pedersen](https://github.com/davidpdrsn) jumped into the fray to help make Tower so much more. The blog post [“Inventing the Service trait”](https://tokio.rs/blog/2021-05-14-inventing-the-service-trait) helps explain the _why_: why have a `Service` trait, and why does it look the way it does. Afterwards, the [tower-http](https://tokio.rs/blog/2021-05-announcing-tower-http) library, which was just a few HTTP-specific middleware with rough edges, was expanded, polished, and shined to a nice finish.

Then came [Axum](https://tokio.rs/blog/2021-07-announcing-axum). Axum is a Rust server framework designed to extend from Tower and Tower-HTTP. While [warp](https://seanmonstar.com/blog/warp/) was always able to work with Tower, it’s main `Filter` API was very opinionated, and felt foreign to those less-used to functional programming. Axum lives next to warp, more fully embracing the `Service`, and using a much more common `Router` style.

### Console

In the second half of 2021, several of us worked a lot on a really exciting tool that will help debug asynchronous applications. In December, we launched the [first release of the Tokio Console](https://tokio.rs/blog/2021-12-announcing-tokio-console). It’s built on top of some familiar technologies, such as `tracing`, `hyper` (via [`tonic`](https://tokio.rs/blog/2021-07-tonic-0-5)), and a scheme for _any_ runtime to report events about tasks and resources.

[![console screenshot](https://raw.githubusercontent.com/tokio-rs/console/main/assets/warnings.png)](https://tokio.rs/blog/2021-12-announcing-tokio-console)

It should be especially useful for service developers, such as those using hyper for their server, as it will provide insight into how the async tasks and resources (mutexes, semaphores, channels, sockets, etc) are behaving. It can help find tasks that are stuck, or slow, or inefficient.

* * *

1. 

hyper-ish, if you will.&nbsp;[↩︎](#fnref:1)

