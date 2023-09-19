---
layout: post
title: 'This Month in hyper: August 2022'
date: '2022-08-31T13:34:34-04:00'
tags:
- rust
- rust-lang
- hyper
- tower
- monthly
- http
- http3
tumblr_url: https://seanmonstar.com/post/694122996505493504/this-month-in-hyper-august-2022
---
Another hot summer month has gone by. Besides many of us convening for RustConf, a lot of wonderful contributors made great progress around [hyper](https://hyper.rs).

### 10,000 stars!

At the beginning of the month, we celebrated [hyper reaching 10,000 stars](https://seanmonstar.com/blog/2022-08-02-hyper-10000-stars/) on GitHub! ⭐

### Releases

#### h2 v0.3.14

We released v0.3.14 of `h2`, the HTTP/2 dependency of hyper. Besides some internal refactoring, the user facing fix is that a server which responds early and wants to ignore the body will now send a `NO_ERROR` instead of a `CANCEL`, as the spec recommends, thanks to @erebe!

#### httparse v1.8.0

A new version of the HTTP/1 parser of hyper came out as well. @nox added an option to ignore invalid header lines, as browsers frequently do. And work from @lucacasonato and @AaronO improved request parsing performance by about ~13%!

### hyper 1.0

We cruised on forward towards the stable hyper 1.0. Part of that is pushing the less stable (higher-level, more opinionated) features to `hyper-util`. So, I removed `hyper::client::service`, `hyper::client::connect`, `hyper::Client`, and `hyper::Server`. @MrGunflame removed the `tcp` feature, and it’s related code `connect` and `accept` code.

The `body` module is also getting some cleaning. @Michael-J-Ward renamed `hyper::Body` to `hyper::body::Recv` temporarily (want to [bikeshed the new name](https://github.com/hyperium/hyper/issues/2971)?), so that @RajivTS could rename `hyper::body::HttpBody` to simpler `hyper::body::Body`. @Xuanwo removed the “full” constructors from `Body`, and @oddgrd made `Body::channel` private, since those are now separate types in `http-body-util`. I wrote a proposal for a [forwards-compatible `Body` trait](https://github.com/hyperium/hyper/issues/2840), and published an article as a preface to the [pattern matching compatibility](https://seanmonstar.com/blog/2022-08-25-pattern-matching-and-backwards-compatibility/) issues being solve in that proposal.

For additions, I added version-specific `Connection` types for the client, to replace the “either-version” type already there. We also put our [governance](https://github.com/hyperium/hyper/blob/master/docs/GOVERNANCE.md) down in writing.

You can follow along (and join us!) on this [dashboard](https://github.com/orgs/hyperium/projects/1/views/6). The goal is to have a release candidate out in September!

### HTTP/3

@eagr got a [spec compliance report rendered](https://hyper.rs/h3/ci/compliance/report.html), using s2n-quic’s [`cargo duvet`](https://crates.io/crates/duvet). This lets us track via comments in the code how much of the HTTP/3 spec we are obeying. We still need to fill in a bunch of comments, but there’s some in there already!

I created a [tower-h3](https://github.com/seanmonstar/tower-h3) repository to experiment with a standalone `h3` client and server that can use Tower middleware. @kckeiks wrote up a magnificient PR to [add experiemental HTTP/3 support to reqwest](https://github.com/seanmonstar/reqwest/pull/1599). The blocker to merging is figuring out the best way to publish “experimental” support to crates.io.

### Tower

We’ve heard from many sources that doing retries correctly is hard. Tower has had a `retry` middleware, but it clearly wasn’t doing enough to get proper retries into people’s applications. So, @LucioFranco [wrote up an issue](https://github.com/tower-rs/tower/issues/682) outlining the ways we’ll make it better. This includes some middleware changes to clarify concepts, and additional utilities so that designing custom retry policies is easy, and pushes the user towards safe defaults.

So far, Lucio has improved the `Policy` trait, derived a simple `rng` utility, and added a `backoff` module with trait and a good default based on exponential backoff.

