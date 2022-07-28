---
layout: post
title: A hyper update
date: '2017-01-20T09:00:03-08:00'
tags:
- programming
- hyper
- http
- rust
- rust-lang
- mozilla
- planet
tumblr_url: https://seanmonstar.com/post/156128815358/a-hyper-update
---
### hyper 0.10

A new version of [hyper](https://hyper.rs) was released last week, [v0.10.0](https://github.com/hyperium/hyper/releases/tag/v0.10.0), to fix a lot of dependency-related problems that the 0.9 versions were having. The biggest problem was version incompatibilities with OpenSSL. You can read [all about the problem and solution in the issue tracker](https://github.com/hyperium/hyper/issues/985), but the **tl;dr** is that hyper 0.10 no longer depends on OpenSSL, or any TLS implementation really. TLS is a big part of HTTP, but there are also several different implementations, and they all release on their own schedules. Even just an optional dependency on OpenSSL could cause unrecoverable dependency conflicts.

This should be the last feature release of hyper using **blocking IO**. You can sort of think of it as an LTS version. If there are serious bugs or security fixes, a point release will be made. But all development is completely focused on the new release that will be using tokio. Speaking of…

### hyper and tokio

The work to make hyper use **non-blocking IO** has been a long road. In recent months, it has been with the help of the tokio library, which just [recently released a version](https://tokio.rs/blog/tokio-0-1/). We just merged the tokio branch into master this week!

> `wrk -t 10 -d 10s -c 20`: 225759.00 requests per second<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>

The full pipeline works great, and it’s fast!<sup id="fnref:2"><a href="#fn:2" class="footnote-ref" role="doc-noteref">2</a></sup> Using futures feels very natural when programming with the asynchronicity of HTTP messages. Instead of including inline code examples here that may grow stale, I’ll just point to the [examples in the hyper repository](https://github.com/hyperium/hyper/tree/master/examples). And yes, full guides will be coming soon, as part of the 0.11 release.

There’s still things to touch up. We’re still trying to find the best ways to a) setup an HTTP server or client for easy use, b) plug in an HTTP server or client into a the wider tokio ecosystem. There’s still internal performance things we could do to get even faster. But there’s a light at the end of this tunnel. It’s growing. If you’d like, [join in!](https://github.com/hyperium/hyper/milestone/3) Or try to port your framework to use it, and provide feedback.

Soon, we’ll have a much better answer for [are we web yet?](http://arewewebyet.org)

* * *

1. 

The benchmarks are hand-wavey at the moment. I surprisingly don’t have an environment available to me to benchmark a bunch of different settings and against other HTTP libraries. If you’d like to help record some benchmarks, I’d greatly appreciate it.&nbsp;[↩︎](#fnref:1)

2. 

Of course, the biggest benefit of non-blocking IO is that it is the best way to scale when you have other IO to do in order to serve a request (files, databases, other networking, etc), or when the payloads are bigger than the the network packet size, and you want to serve thousands of those requests at the same time.&nbsp;[↩︎](#fnref:2)

