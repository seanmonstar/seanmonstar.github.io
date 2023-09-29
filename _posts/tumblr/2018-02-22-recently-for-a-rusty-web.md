---
layout: post
title: Recently, for a Rusty Web
date: '2018-02-22T13:51:14-05:00'
tags:
- rust
- rust-lang
- hyper
- buoyant
tumblr_url: https://seanmonstar.com/post/171170905822/recently-for-a-rusty-web
---
It’s been a few months since [I shifted my focus](http://seanmonstar.com/blog/bye-mozilla-hello-buoyant/) full time to Rust, and in that time, we’ve gotten a lot of work done! I wanted to update you here what all that is, since it’s spread around on multiple repositories.

## The `http` crate

We [started working](https://users.rust-lang.org/t/announcing-the-http-crate/12123) on the [http](https://crates.io/crates/http) crate earlier last year, but in September, we released [v0.1](https://crates.io/crates/http)! The point of this crate was pull out the “HTTP primitives”, like headers, methods, status codes, and the like, and put them into a base crate. The hope is that HTTP implementations can expose APIs using these types, and web frameworks can expect them, and thus more easily change implementations if so desired.

Since then, we’ve seen several implementations already put them to good use:

- [h2](https://crates.io/crates/h2)
- [actix-web](https://crates.io/crates/actix-web)
- [simple-server](https://crates.io/crates/simple-server)

## The `h2` crate

At [Buoyant](https://buoyant.io), one of the products we build is [Conduit](https://conduit.io), a service-mesh slash proxy built for Kubernetes environments, requiring minimal setup. The proxy part (“data plane”) is built in Rust. Since many internal services take advantage of HTTP2, we needed a working HTTP2 implementation in Rust. So, we built one! After building and using it in Conduit, we eventually got it to a [v0.1 release of h2](https://crates.io/crates/h2).

Importantly, it’s fast, implements the full HTTP2 specification, and works with asynchronous IO and futures.

## Tower and gRPC

Besides an HTTP2 implementation, we also needed use [gRPC](https://grpc.io) to have the proxy talk with the controller. We built up a middleware suite, [tower](https://github.com/tower-rs/tower), which needs its own announcement when it’s released, but in short, is an evolution of [tokio-service](https://crates.io/crates/tokio-service), sort of like finagle for Rust. Once we had several `Service`s working, we got to work on [tower-grpc](https://github.com/tower-rs/tower-grpc), which merges together tower, the h2 crate, and protobuf to get elegant code generation speaking gRPC.

## hyper

Besides all those awesome things, the [hyper](https://hyper.rs) crate has continued to see increased development. Since September, hyper has seen 16 point releases, bringing bug fixes, performance improvements, and new features! Some highlights:

- **hyper got even faster.** Besides bug fixes and new features, hyper managed to get even faster than before, as shown in these [benchmarks](https://www.techempower.com/benchmarks/#section=data-r15&hw=ph&test=plaintext). Admittedly, microbenchmarks need to be considered with plenty of salt, but it ain’t nothing! Besides the microbenchmark, hyper also improvement throughput for bigger streaming bodies by intelligently using `writev` when it can (it also detects if `writev` isn’t helping, and reverts to previous behavior).
- **Added APIs to create servers with a shared tokio `Core`**. The easy entry point to running a server in hyper is using `Http::bind`, which creates a `Server` with its own `Core`. However, as applications get more complex, you may want to use the same `Core` for multiple things. Several APIs were added to allow doing just that: `Http::serve_addr_handle`, `Http::serve_incoming`, and `Http::serve_connection`.

- **Removed dependence on tokio-proto.** The `tokio-proto` crate was initially used internally, but it hasn’t seen many updates, and it was found to be buggy. hyper was suffering from those bugs, as well as not being able to improve the exposed API. A bunch of work was done to build an internal dispatcher, removing the need to rely on tokio-proto, and fixing a bunch of bugs in the process.
- **Added compatibility with the `http` crate.** Completely switching the HTTP types to those from the `http` crate is a breaking change, but to allow usage incrementally, you can opt-in to using them by enabling the Cargo `compat` feature of hyper. In 0.12, they will be completely replaced.

## Coming soon

With all the work happening to release [futures 0.2](https://github.com/rust-lang-nursery/futures-rfcs/blob/master/futures-02.md), the [tokio reform](https://tokio.rs/blog/2018-02-tokio-reform-shipped/), the [http crate](https://crates.io/crates/http), and the [h2 crate](https://crates.io/crates/h2), there are some changes coming to hyper. See the [0.12 milestone](https://github.com/hyperium/hyper/milestone/4) if you want to follow along!

