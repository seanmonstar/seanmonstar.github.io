---
layout: post
title: 'This Month in hyper: October 2022'
date: '2022-11-01T14:47:12-04:00'
tags:
- hyper
- rust
- rust-lang
- http3
- monthly
tumblr_url: https://seanmonstar.com/post/699744578823127040/this-month-in-hyper-october-2022
---
As the leaves change and fall, our wonderful contributors continue to make [hyper](https://hyper.rs) ever better!

### Releases

#### hyper v1.0.0-rc.1

After so much work through this year, we finally published the first release candidate for hyper 1.0, [hyper v1.0.0-rc.1](https://seanmonstar.com/blog/2022-10-26-hyper-v100-rc1/)!

The community had some [wonderful things](https://www.reddit.com/r/rust/comments/ydzedc/hyper_v100_release_candidate_1/) [to say](https://twitter.com/seanmonstar/status/1585275994235101189):

> Parking in a stable stop is such a lovely metaphor for library evolution, love it!
> 
> Very exciting, this is a huge milestone for the maturity of Rust’s web-facing ecosystem. :)
> 
> all aboard the hype® train! 🚂

As the [announcement post](https://seanmonstar.com/blog/2022-10-26-hyper-v100-rc1/) said, we’ve got more to do. We’re moving into the **hyper polish period**. I’ll have more to say about that soon! But you can [join us in chat now](https://discord.gg/kkwpueZ) if you want to help out.

#### hyper v0.14.21

We also published [v0.14.21](https://github.com/hyperium/hyper/releases/tag/v0.14.21), to bring some fixes and features to the more stable branch. This included advanced TCP `Server` options, an option to ignore invalid header lines, and some more bug fixes.

Part of the 1.0 plan is to backport as much as possible to 0.14, in an effort to make upgrading easier. So you’ll still see 0.14.x releases along the way.

### hyper 1.0

In order to publish the first release candidate, [hyper v1.0.0-rc.1](https://seanmonstar.com/blog/2022-10-26-hyper-v100-rc1/), there was a bunch of work to finish up.

@Michael-J-Ward created the per-version `Connection` types for the server module. And then I finished up the split by removing the combined `hyper::server::conn::Connection` type. @bossmc then removed an unneeded `Option` wrapping the internals of `hyper::server::conn::http1::Connection`, and dropped the `try_into_parts` method.

@LucioFranco refactored out the `hyper::body::aggregate` functions into a `Collected` type in `http-body-util`. I was able to use those to upgrade hyper with the [new `Body` trait design](https://github.com/hyperium/hyper/issues/2840) that works on frames, making it forwards-compatible. I also finished up the bike-shaving to determine the name of the body type hyper returns, settling on `hyper::body::Incoming`.

### hyper in curl

@dannasman cleaned up a feature we ended up not needing: the ability to get the raw response bytes. curl ended up preferring using the parsed response fields, keeping the parsing in Rust.

Are you interested in [helping to debug](https://seanmonstar.com/blog/2022-03-16-help-stabilize-hyper-in-curl/) the [last few unit tests](https://github.com/orgs/hyperium/projects/2/views/1) for hyper in curl?

### HTTP/3

We’re working on HTTP/3 in a separate crate, [h3](https://github.com/hyperium/h3), with the goal of fitting it into [hyper](https://hyper.rs).

@g2p documented the entire public API for the `h3` and `h3-quinn` crates. @eagr refined the compliance report generation, and exception reasons. In [tower-h3](https://github.com/seanmonstar/tower-h3), @eager made use of the new `Body` trait from the hyper rc1, and better use of `Endpoint`.

### Tower

[Tower](https://github.com/tower-rs/tower) (and `tower-http`) are a protocol-agnostic RPC framework with middleware, and they combine nicely with hyper.

@jplatte implemented `Layer` for tuples up to 16 elements, such that a tuple of layers is similar to using `ServiceBuilder`. @samvrlewis added methods to `ReadyCache` to allow iterating over the ready services, and implemented `Clone` for`discover::Change`. @boraarslan added a `trait Budget`, so that the [retry middlware revamp](https://github.com/tower-rs/tower/issues/682) can allow swappable budget implementations.

@82marbag introduced a new `TimeoutBody` middleware to wrap HTTP requests and/or responses in a body type that will timeout reads. This adds onto the existing main timeout middleware, which just times out waiting for response headers.

