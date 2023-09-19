---
layout: post
title: 'This Month in hyper: March 2023'
date: '2023-04-07T09:31:29-04:00'
tags:
- monthly
- http3
- hyper
- rust
- rust-lang
tumblr_url: https://seanmonstar.com/post/713948439199858688/this-month-in-hyper-march-2023
---
The days are growing longer up here, and work to make [hyper](https://hyper.rs) better and better continues onwards!

There was some particularly exciting releases, and a bunch of conversations had in March 2023.

### hyper 1.0

#### Preparing users for upgrading to 1.0

One of the important parts of the [hyper 1.0 polish period](https://seanmonstar.com/blog/2022-11-15-hyper-polish-period/) is making sure _upgrading_ from 0.14 to 1.0 is as smooth as we can make it. [hyper v0.14.25](https://github.com/hyperium/hyper/releases/tag/v0.14.25) is here to help. @kxt and @oddgrd backported the client and server APIs from 1.0, and added opt-in deprecation messages to help you be ready to upgrade. The deprecation warnings are meant to help you, not to annoy you.

You can enable them to see where you can start preparing your code now:

    [dependencies]
    # besides whatever other features you've enabled...
    hyper = { verion = "0.14.25", features = ["backports", "deprecated"] }

#### RC4 discussions

We’ve been discussing how to wrap up the last couple of changes for 1.0, to put out an RC4 to bake some. We have some decent answers, and can get to work.

hyper’s own [`Service` trait will change from `&mut self` to `&self`](https://github.com/hyperium/hyper/issues/3040). This better aligns with reality, as most services already need to share state behind some synchronization mechanism, and needing to `&mut self` in the method call doesn’t help.

hyper will [use its own IO traits](https://github.com/hyperium/hyper/issues/3110) with forwards-compatibility in mind. We want to be able to support both poll-based and completion-based (think `epoll` vs `io-uring`) IO models.

### HTTP/3

We’re working on HTTP/3 in a separate crate, [h3](https://github.com/hyperium/h3), with the goal of fitting it into [hyper](https://hyper.rs).

#### reqwest includes experimental HTTP/3 support

With [reqwest v0.11.15](https://github.com/seanmonstar/reqwest/releases/tag/v0.11.15), you can try out HTTP/3, on the client side, in reqwest _right now_! It’s currently experimental, which means a couple things: it might not work perfectly. Let us know! It also might be disabled in new patches, as we fiddle with it. Lastly, you need to more explicitly opt-in to the instability.

That means that besides enabling the `http3` feature from your `Cargo.toml`, you also need to pass `RUSTFLAGS="--cfg reqwest_unstable"` to the compilation job.

Huge thanks to @kckeiks for integrating `h3` into `reqwest`!

#### h3-quinn upgraded to Quinn 0.9

We found a way to upgrade h3-quinn, using `stream::unfold` and `BoxStream`.

The `h3` crate tries to be generic over any QUIC implementation. The `h3-quinn` crate implements the `h3::quic` traits for the [Quinn crate](https://crates.io/crates/quinn). The traits currently use poll-based methods (async function in traits isn’t stable yet, and they also wouldn’t allow polling multiple things at the same time). Newer Quinn embraced using `async fn` on its types, instead of returning named futures, which is very fair thing to do. But it did make it hard to figure out how to implement poll-based methods over `async fn`s.

I realized we could do a async-move-dance to solve this. We make an `async move` block, moving in the type and awaiting the future, which then returns a tuple of the original type and the return value. @Ralith made the suggestion to use `stream::unfold`, which streamlines that pattern. Then @inflation quickly wrote it up, and we were able to upgrade to Quinn v0.9.

### Contribute

Want to help us out? Even trying the new releases out and give us feedback is extremely useful. Of course, contributing reviews is a great help too. Come by and [say hi](https://discord.gg/kkwpueZ)!

