---
layout: post
title: "reqwest v0.13 - rustls by default"
excerpt: "reqwest v0.13 brings rustls by default, feature cleanup, but otherwise easy to upgrade."
date: '2025-12-30T10:48:00'
tags:
- rust
- reqwest
- open-source
- programming
---
To end out the year, here comes a new major release of [reqwest][], the opinionated higher-level HTTP client for Rust.

We don't really need major breaking versions to keep providing value. Improvements keep coming all the time. But we did need one to make one particular big adjustment, and we've taken the opportunity to clean up other things too. At the same time, we strove make it disrupt as little as possible, especially if you stick to the defaults.

### rustls is now the default TLS backend

The biggest deal is that reqwest now sets its default TLS feature to use [rustls][], instead of native-tls.

Granted, native-tls has it's place. It provides a unified library that uses the "native" TLS implementation on each target. SecureTransport on macOS, schannel on Windows, and OpenSSL on Linux (as the most likely already installed). It was the right choice several years, while rustls was young. But rustls is now safer and faster than most choices.[^windows-i686-gnu] Seems like an obvious [improvement][]. But would people want the better choice?

A recent hyper user survey found that 93% of respondants already use rustls. 30% said they also use native-tls at times, so we will continue to provide that option. And while a survey is already biased, it's also most certainly true that the vast majority of users just allow the default options, and it works for them. That will continue to be true for most everyone.

So, if you use the default options, things just got better for you.

### Certificate verification features are consolidated

Previously, reqwest had many crate features to enable various ways to load sources of root certificates. Too many. It's a bit of a mess.

We consolidate the options into just a few simple choices:

- Defaults to using the native platform verifier.
- If the target supports it, `tls_certs_merge()` can add additional certificates. If it doesn't, a builder error is returned.
- If you really need a specific certificate, use `tls_certs_only()`, which will always work, and doesn't include native verification.

These basic options should allow for any use case, while reducing complexity within reqwest. For instance, if an application really needs to use the webpki-roots, they can be configured with `tls_certs_only(your_roots)`.

### Soft-deprecation to improve option naming

While in there, most `ClientBuilder` methods were given better names. The previous methods have been soft-deprecated. That means that you can keep using the old names, without warnings, but they are documented as deprecated, and they will eventually be removed in a later breaking change.

But in most cases, the improved name is mostly to help with understanding. It doesn't help to trigger a bunch of additional warnings when you upgrade. Perhaps we'll add a `deprecations` crate feature for those who want to clean up.

The upside is that all the methods now start with `tls_`. They are grouped together in the docs, but it will also help exploration with autocomplete.

### Other crate feature adjustments

As this is a breaking change version, we did include one making the `RequestBuilder::query()` and `form()` methods optional features, disabled by default. With these as optional, it is now possible to build reqwest without serde.

We also made `native-tls` imply ALPN automatically. This will mean most people who don't think about it will now get HTTP/2 upgrades. The reason it was opt-in previously is because older native libraries might not have the symbols required. They are quite old at this point. If you still need the previous behavior, there is now `native-tls-no-alpn`.

### Thanks!

Thanks to all who contribute, use, [sponsor][], fix, complain, and help reqwest be what it is! Here's [v0.13.0][].


[^windows-i686-gnu]: This does drop `i686-pc-windows-gnu` support for the default features. The reality is that a _lot_ of things don't work for that target. Even Rust itself has it as a Tier 2 target. We do still support it with native-tls, though.

[reqwest]: https://crates.io/crates/reqwest
[rustls]: https://rustls.dev/
[improvement]: https://github.com/seanmonstar/reqwest/issues/2025#issuecomment-2913873836
[sponsor]: https://seanmonstar.com/sponsor
[v0.13.0]: https://github.com/seanmonstar/reqwest/releases/tag/v0.13.0
