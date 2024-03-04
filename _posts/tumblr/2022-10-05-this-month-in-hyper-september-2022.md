---
layout: post
title: 'This Month in hyper: September 2022'
date: '2022-10-05T13:30:02-04:00'
tags:
- rust
- rust-lang
- hyper
- monthly
- http
- http3
tumblr_url: https://seanmonstar.com/post/697293605419745280/this-month-in-hyper-september-2022
---
And I thought _last_ month was hot, this one was a scorcher (on the US West Coast[^1]). but just look at all the wonderful contributors making [hyper](https://hyper.rs) ever better!

### Releases

#### reqwest v0.11.12

A new version of [reqwest, v0.11.12](https://github.com/seanmonstar/reqwest/releases/tag/v0.11.12), includes a way to override DNS with multiple addresses (thanks @lpraneis), support for HTTP upgrades such as websockets (thanks @luqmana), and some internal maintenance (thanks @futursolo, @kckeiks, and @vidhanio).

### hyper 1.0

The march towards the [first release candidate](https://github.com/orgs/hyperium/projects/1/views/6) brings ever closer!

@Robert-Cunningham added the `hyper::rt::Timer` trait and integration, and @oddgrd removed the `runtime` cargo feature, since everything now uses the `Timer` trait. @tomkarw replaced hyper’s usage of `tower::Service` with a hyper-specific `Service` trait. @Michael-J-Ward removed the combined `hyper::client::conn::Connection` type. hyper will contain version-specific `Connection` types, and a “combined” interface will grow in `hyper-util`.

I streamed on [twitch.tv/seanmonstar](https://twitch.tv/seanmonstar) briefly about putting together a launch checklist, since we’re so close to the [first release candidate](https://github.com/orgs/hyperium/projects/1/views/6)!

### hyper in curl

I gave a [talk for curl-up 2022](https://seanmonstar.com/blog/curl-up-2022-hyper-in-curl/).

@weihanglo updated hyper’s C API to the now-stable `cargo rustc --crate-type cdylib`, no longer requiring nightly.

### HTTP/3

@Ruben2424 got `h3spec` running in CI, and fixed up things it caught. @eagr signficantly filled out the rest of the [spec compliance report](https://hyper.rs/h3/ci/compliance/report.html), including exceptions and related unit tests. @g2p documented the whole public API, and removed the unused crate dependencies.



[^1]: I’m actually moving across the country, which is why this update was delayed.

