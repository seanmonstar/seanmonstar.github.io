---
layout: post
title: hyper-ish 2022 in review
date: '2023-01-18T11:28:08-05:00'
tags:
- yearly
- hyper
- http3
- curl
- rust
- rust-lang
- tower
tumblr_url: https://seanmonstar.com/post/706802392260362240/hyper-ish-2022-in-review
---
Quite the eventful year! With 2022 over<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>, I want to take some time highlight what’s happened in [hyper](https://hyper.rs) and the immediately surrounding landscape.

A quick personal note, I wrote much more this year!<sup id="fnref:2"><a href="#fn:2" class="footnote-ref" role="doc-noteref">2</a></sup> Yay!

### hyper

Let’s start with some general things about [hyper](https://hyper.rs).

On the human side, there were 62 unique contributors to hyper in 2022. We added [1 more collaborator](https://hyper.rs/blog/2022/04/08/welcome-adam-foltzer/) and [2 triagers](https://github.com/hyperium/hyper/blob/master/docs/MAINTAINERS.md#triagers), and even defined what those [roles](https://github.com/hyperium/hyper/blob/master/docs/GOVERNANCE.md) even mean.

In an effort to share the lovely work those humans do, I started writing [monthly hyper updates](https://seanmonstar.com/tagged/monthly).

And we celebrated [10 thousand stars](https://seanmonstar.com/blog/2022-08-02-hyper-10000-stars/) this year! ⭐

### hyper 1.0

As I mentioned in [last year’s review](https://seanmonstar.com/blog/2022-01-04-hyper-ish-2021-in-review/), 2022 was the year we began to prepare to release a stable 1.0 of [hyper](https://hyper.rs).

We started with a [v1.0 timeline](https://seanmonstar.com/blog/2022-02-22-hyper-10-timeline/) (of which we’re in the final steps). After much discussion with users, we put into writing [hyper’s VISION](https://seanmonstar.com/blog/2022-03-08-hypers-vision/), which defined _where we were going_. Shortly aftewards came the [v1.0 ROADMAP](https://seanmonstar.com/blog/2022-04-06-hyper-10-roadmap/), outlining _how to get there_.

And then, a summer flurry of coding and hairy diffs.

On the other side, late last year, we published [v1.0.0-rc.1](https://seanmonstar.com/blog/2022-10-26-hyper-v100-rc1/). This represented the _likely_ working library that we’d like set down into a stable parking spot for a while. While people check it out and give us feedback, we entered the [hyper polish period](https://seanmonstar.com/blog/2022-11-15-hyper-polish-period/), making sure everything _feels_ good. That’s where we still are, for a short period more.

You can follow along on the [project board](https://github.com/orgs/hyperium/projects/1/views/1), and specifically the [1.0 meta issue](https://github.com/hyperium/hyper/issues/3088) ties together non-issue related details.

### hyper in curl

The work to make [hyper an HTTP backend in curl](https://seanmonstar.com/blog/2021-09-16-how-using-hyper-in-curl-can-help-make-the-internet/) from last year continued throughout this year. There’s just a few remaining tests in curl’s large HTTP suite that didn’t work when hyper is enabled. Several wonderful people showed up to dig in and find out exactly _why_.

To try to empower others to do, I wrote up a [help-us-finish guide](https://seanmonstar.com/blog/2022-03-16-help-stabilize-hyper-in-curl/), explaining step-by-step how anyone could help us finish this all-important work. Later in the year, I [streamed](https://twitch.tv/seanmonstar) a [hyper-in-curl debug session](https://youtu.be/p45KggejkJ4), where you could watch me follow the guide, and then bash the keyboard randomly while trying to understand what the issue is. You can still watch the [recording](https://youtu.be/p45KggejkJ4), or see [these details](https://masto.ai/@seanmonstar/109428187573196836) about what test was solved and the pull requests that came out of it.

I also joined Daniel Stenberg virtually by presenting about [hyper in curl at curl-up 2022](https://seanmonstar.com/blog/2022-09-21-curl-up-2022-hyper-in-curl/).

### h3

We’ve been working on the [h3 crate](https://github.com/hyperium/h3), providing HTTP/3 that is generic over any QUIC implementation, with the goal of integrating into hyper directly. The repository has had a working server and client which already interoperates with other implementations.

I’ve written about it in the [monthly](https://seanmonstar.com/tagged/monthly) updates, but here’s some highlights I’m excited about:

- We added 3 new collaborators who have been driving the work: @eagr, @g2p, and @Ruben2424.
- @stammw implemented graceful shutdown for the server and the client.
- @Ruben2424 added [GREASE](https://textslashplain.com/2020/05/18/a-bit-of-grease-keeps-the-web-moving/) support via an default-on option which sends random reserved identifiers, such as frames, settings and streams, to help prevent ossification that would make future extensions harder.
- @eagr made it so we now track compliance with the HTTP/3 specifications, by using special comments that are compared with the spec text, and it even outputs [a report](https://hyper.rs/h3/ci/compliance/report.html) updated as part of our continuous integration.
- @Ruben2424 also added `h3spec` to CI, and fixed the missing pieces it noticed.
- @g2p documented the entire API.

While there’s still specific work that can be done on the `h3` crate itself, it’s time to consider next steps to get it into user hands. To that end, there’s even a [pull request for reqwest to use `h3`](https://github.com/seanmonstar/reqwest/pull/1599)! There’s just some [details to work out](https://github.com/hyperium/h3/issues/125) around publishing unstable versions so reqwest can depend on it. We hope this experimental support will help us iron out any usage annoyances, so we can start landing it in hyper proper.

### tower

While [tower](https://crates.io/crates/tower) isn’t tied to hyper, we’ve always meant for people to easily combine the two libraries to make powerful, opinionated HTTP stacks.

In the later half of the year, we started having discussions about making tower _easier_. Up until now, it has mostly felt like expert mode. But if done right, we shouldn’t be telling users “no, you don’t hold it _that_ way” when they try to implement retries slightly wrong and storm their servers. So, to that end, Lucio put together a big issue outlining how we can make [retries better](https://github.com/tower-rs/tower/issues/682).

Another discussion started about the [path to tower 1.0](https://github.com/tower-rs/tower/issues/636). This brought some interesting questions around how `Service` handles backpressure, whether we could make _that_ any better. Certainly, something else we would want to consider is if `Service` can make use of [async fn in traits](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html).

The `tower-http` repository continued to see additions. Mostly middleware that are specific to HTTP that many people would find beneficial, such as `RequestBodyLimit`, `RequestBodyTimeout`, `ResponseBodyTimeout`, and `ValidateRequest`.

I’ve also kicked around the idea loosely about cracking open the [reqwest](https://crates.io/crates/reqwest) crate, and turning its various features into tower middleware. Then, reqwest is just a single opinionated way to build up a client stack. The community would be more empowered to customize the order of layers, adding or removing or swapping, and still have the power that they come to expect from using reqwest.

### What are some possiblities in 2023?

Besides launch hyper 1.0, of course.

These are are all things that many people have asked for, and I’d like to see done. But realistically, most will require help from you!

- Improved middleware
- HTTP/3 in hyper
- Tracing and Metrics
- `h2` performance improvements
- An even-lower level `http1` codec crate
- Tower-ify `reqwest`

I’ll likely be focused at the top of that list, but would welcome anyone interested jumping into an issue (or discussing on [Discord](https://discord.gg/kkwpueZ) if you prefer). Really, the biggest success would be [empowering others](https://github.com/hyperium/hyper/blob/master/docs/GOVERNANCE.md) to be the leaders and owners on these things. Do you want to be one of them?

* * *

1. 

Sorry for the delay, illness struck right at the beginning of the year.&nbsp;[↩︎](#fnref:1)

2. 

I wrote barely anything in 2021, 2020… actually for quite a few years. I used to blog multiple times a month back in 2013, but kind of teetered off the following year. Anyways, I’m really liking it (again), so here’s to more!&nbsp;[↩︎](#fnref:2)

