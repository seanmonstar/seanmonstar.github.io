---
layout: post
title: A trait for fluent Durations
tags:
- rust
bsky_url: https://bsky.app/profile/did:plc:ax5sh6hthwrelunhn2bfmrsb/post/3msvew6rwcw2d
mastodon_url: https://masto.ai/@seanmonstar/117083096961219861
---
I dislike the pattern in some languages to create durations by multiplying constants. It feels like a concession when it cannot be expressed more nicely. There's a [tracking issue](https://triagebot.infra.rust-lang.org/gh-comments/rust-lang/rust/issues/57391) to add such constants in libstd.

How about a trait instead? (I suggested it in the tracking issue a long time ago, but it's lost in the noise). Rust traits are awesome. They can be implemented on any other type, even primitives, without them needing to cooperate.

A trait could allow us to write `5.seconds()`, or `200.milliseconds()`, etc. I think this is much better. I would rather this exist instead. Maybe `std::time::TimeUnits`, or pick a better name, doesn't matter which, just that it's easy to import.
