---
layout: post
title: "Podcast: Netstack.fm, story of Rust's networking with hyper"
date: '2025-09-02T09:24:00'
tags:
- rust
- hyper
- podcast
- open-source
---
Last week I was a [guest on the Netstack podcast][podcast]. We talked abit about how I got into Rust, how async Rust developed, and the story behind [hyper][] and its surrounding ecoystem.

We started (and ended) with my goal of better software:

> _On your about page, you say that “Rust is the least bad option.” Can you elaborate a bit on that?_
>
> Yeah, I love Rust. I think Rust solves a ton of problems, but I also don't want to be stuck when something better comes along, which it inevitably will. Then yeah, I'll move to that.
> To me, all these tools are the means to an end, which is to make better software.

The rest of the conversation was really fun to talk about:

- _7:54_: beginning of hyper, async Rust
- _13:20_: hyper as something bigger
- _15:36_: splitting off hyper-util
- _17:35_: the `http` and `headers` crates
- _22:51_: motivation behind [warp][]
- _29:00_: [reqwest][] as the opinionated layer
- _30:17_: open source [independence][]
- _31:31_: HTTP/3 in hyper and reqwest
- _39:40_: hyper in 5 years, contributors, ownership

[podcast]: https://netstack.fm/#episode-2
[hyper]: https://hyper.rs
[warp]: /blog/warp-v04
[reqwest]: /blog/modular-reqwest
[independence]: /blog/independent-open-source-maintainer
