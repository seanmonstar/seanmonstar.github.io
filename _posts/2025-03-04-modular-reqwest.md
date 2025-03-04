---
layout: post
title: 'A More Modular reqwest'
date: '2025-03-04T13:05:00'
tags:
- rust
- reqwest
- tower
- http
- open-source
- programming
---
reqwest came out [8 years ago](https://seanmonstar.com/blog/introducing-reqwest/). It was meant to be a higher-level HTTP client, with batteries and opinions included. And it continues to fulfill that role very well.

To do so, reqwest combined or implemented a lot of features that weren't easily accessible elsewhere in Rust. Things like redirect handling, connection proxies, and compression.

People love it. It's the most popular HTTP client used in Rust code. As of today, it gets downloaded 10 million times per month from crates.io (which doesn't include any internal mirrors). It helps engineers all around the world.

People also wish they could better customize it. They want to add in custom logic somewhere. This makes sense. And the ecosystem has gotten [even better at reuse][tower-http]. It's time for reqwest to help be more reusable.

First, what is the goal?

### Easy and modular

Top priority is that reqwest remain **easy** to use. It should be the best default for most people. If you don't know what customizations you want, you shouldn't have to care.

It should also be _possible_ to customize. To tinker. To grab just a single piece. To re-order the pieces. To add in completely new pieces.

### Some concrete work

So, how do we get there? Well, here's a start.

The connection pool can be changed from a single Pool type into a series of `Service`s that allow better composition, such as a racing cache pool, singleton pool for HTTP/2, an `Either` pool to combine them. This allows people to use less complicated pools than one that includes connection racing. It makes way for using tower's load balancing services directly within reqwest, even.

The HTTP Proxy internals can be refactored to be another `Service` which fits into the series of "connector" services reqwest uses. These include loading proxy data from environment variables, and providing HTTP/HTTPS/SOCKS proxies and tunneling. Other applications that only use reqwest for proxying can depend on only those utilities.

reqwest supports some basic redirect policies, and allows for custom ones. Eventually, `tower-http` added middleware to do that too. We can now use redirects from tower-http. It'd be better to update reqwest to depend on that middleware internally, allowing reqwest's test suite to find any pieces lacking in the port.

We can add _easy_ request retries. reqwest doesn't really have retries, other than a very specific case: special HTTP/2 error cases. And conceptually, retries aren't that different from redirects. Retrying requests is pretty common thing to ask for, and it's pretty easy to do them wrong. There's middleware in `tower::retry`, but it's not easy. We could make it as easy as calling `.retries(reqwest::retry::idempotent())` on a client builder. Or a few other common policies.

We can use decompression from `tower-http`. reqwest has automatic response body decompression. Eventually, it was added as middleware in `tower-http`. Instead of having the feature essentially duplicated, we can just depend on the modular version, and the test suite will keep the quality up. Doing it this way improves the middleware for everyone. For instance, when one user swapped from reqwest's decompression to using tower-http, performance got worse. Eventually, a reused allocation was backported, bringing it back in line.

### Better default TLS

Initially, reqwest made use of `native-tls` for HTTPS support. Eventually, we added optional support for `rustls`. But we've kept `native-tls` as the default. The reason was because it worked for more users, and more sites. However, I think it's time to change the defaults.

At this point, reqwest's defaults actually target the worst common denominator. It not longer feels right to make people opt-in to the better choice. As I said at the beginning, reqwest bundles opinions too.

Our opinion is that reqwest should have the _best_ defaults for the most people. Those that need something different can opt-in to that.[^defaults]

So, reqwest will be changing it's `default-tls` feature to rely on `rustls`. Likely with `aws-lc-rs` as the crypto backend. Optional support for `native-tls` will remain, for now.

The `default-tls` feature theoretically was designed to allow for such a change, since nothing should be exposed with it that isn't supported by `rustls`. But they do have different build requirements, so it won't be 100% smooth.

But it will be better for most people.

### Getting to work

That's the work. Keeping reqwest easy. Allowing more customization. Even letting people use fewer parts of it. While having strong opinions. Making it better for most people.

**Want to be part of it?** Help contribute code or documentation. Here's [the issue plan](https://github.com/users/seanmonstar/projects/3).

You can also [sponsor][] the work. Roadmaps and priorities like these come about from conversations with companies that keep retainers.

All of it helps! To making HTTP requests better!

[^defaults]: This could also be applied to other features too. I think there's an argument to be made for switching the default DNS resolver to use hickory-dns instead `getaddrinfo`.

[tower-http]: https://tokio.rs/blog/2021-05-announcing-tower-http
[sponsor]: https://seanmonstar.com/sponsor
