---
layout: post
title: 'This Month in hyper: February 2023'
date: '2023-03-02T10:38:06-05:00'
tags:
- hyper
- rust
- rust-lang
- monthly
- http3
tumblr_url: https://seanmonstar.com/post/710694914534539264/this-month-in-hyper-february-2023
---
After [recapping the 2022 year](https://seanmonstar.com/blog/2023-01-18-hyper-ish-2022-in-review/), here’s what the amazing contributors have been doing to make [hyper](https://hyper.rs) ever better during January and February of 2023.

### Releases

- [hyper v0.14.24](https://github.com/hyperium/hyper/releases/tag/v0.14.24): fixes some expect-continue behavior, and reduces the internal max allocation in `to_bytes`.
- [h2 v0.3.16](https://github.com/hyperium/h2/releases/tag/v0.3.16): adds a missing piece for Extended CONNECT, and several bug fixes (memory reduction, panics)
- [reqwest v0.11.14](https://github.com/seanmonstar/reqwest/releases/tag/v0.11.14): adds `Proxy::no_proxy(url)` that works like the `NO_PROXY` environment variable, and several internal optimizations reducing copies and memory allocations.
- [tower-http v0.4.0](https://github.com/tower-rs/tower-http/releases/tag/tower-http-0.4.0): a new decompression layer for Requests, `ServeDir` and `ServeFile` now translates IO errors into Responses, and adds a more flexible `ValidateRequest` layer.

### hyper 1.0

We released [RC3](https://github.com/hyperium/hyper/releases/tag/v1.0.0-rc.3), which fixed up some missing pieces in the API. Places needing an `Executor` now ask for one, and we added `hyper::rt::bounds` to publicly expose [nameable but future-proof Executor trait “aliases”](https://github.com/hyperium/hyper/issues/3097), so libraries building on top of hyper can use them as bounds. We also added a few state getters for `SendRequest` which were needed for the next exciting part.

**The higher-level pooling `Client` from 0.14.x has been ported to [hyper-util](https://github.com/hyperium/hyper-util)**. This was the most common blocker preventing people from trying out the release candidates. You can now use the `legacy::Client` with [hyper 1.0.0-rc.3](https://github.com/hyperium/hyper/releases/tag/v1.0.0-rc.3), and have the normal connecting/pooling client experience you’re used to (see the [example](https://github.com/hyperium/hyper-util/blob/master/examples/client.rs)).

We’re still in the [hyper polish period](https://seanmonstar.com/blog/2022-11-15-hyper-polish-period/) 💅. There’s still a little bit of time left to get us your feedback! It’s the _most important_ part of this period. Or join us in one of the four polish areas (or help lead one)! You can also come [chat](https://discord.gg/kkwpueZ) with us about anything.

We took some extra time to focus on [RC4](https://github.com/orgs/hyperium/projects/1/views/6), which has the last few breaking changes to go. Likely, hyper will vendor it’s own IO traits, and change `Service::call` to be `&self` instead of `&mut self`. See the related issues if you have feedback.

The extra time will also allow us to investigate having a security review done for 1.0, to prevent any gotchas.

### HTTP/3

We’re working on HTTP/3 in a separate crate, [h3](https://github.com/hyperium/h3), with the goal of fitting it into [hyper](https://hyper.rs).

Within the next couple days, we’d like to [publish a v0.0.1 to crates.io](https://github.com/hyperium/h3/issues/125). The API likely will change in the very near future, but knowing exactly how requires allowing [experimenters like reqwest](https://github.com/seanmonstar/reqwest/pull/1599#issuecomment-1227414443) to try it out.

