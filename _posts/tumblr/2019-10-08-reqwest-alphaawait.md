---
layout: post
title: reqwest alpha.await
date: '2019-10-08T17:05:02-04:00'
tags:
- rust
- rust-lang
- http
- reqwest
tumblr_url: https://seanmonstar.com/post/188220739932/reqwest-alphaawait
---
[reqwest](https://crates.io/crates/reqwest) is a higher-level HTTP client for Rust. I’m delighted to announce the first [alpha](https://github.com/seanmonstar/reqwest/releases/tag/v0.10.0-alpha.1) release that brings `async`/`await` support!

Some headline features are:

- Add `std::future::Future` support (hello `async`/`await`).
- Add experimental WASM support.
- Change the default client API to async, moving the previous synchronous API to `reqwest::blocking`.
- Make many “extra” features optional to reduce unnecessary dependencies (`blocking`, `cookies`, `gzip`, `json`).

Hey look, a cute example using the new `async`/`await` syntax with `reqwest`:

    dbg!(reqwest::get("https://hyper.rs").await?.text().await?);

These alpha versions are depending on Rust 1.39, which (as of this post) aren’t stable yet. Some other things may change in `reqwest` before the full release (can other features be made optional?), but the alphas allow others to experiment _now_.

My sincere thanks to all that help contribute to `reqwest`! Enjoy `&lt;3`

