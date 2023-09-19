---
layout: post
title: hyper alpha supports async/await
date: '2019-09-04T14:53:52-04:00'
tags:
- hyper
- rust
- rust-lang
- http
- tower
tumblr_url: https://seanmonstar.com/post/187493499882/hyper-alpha-supports-asyncawait
---
I’m excited to announce the [first alpha](https://github.com/hyperium/hyper/releases/tag/v0.13.0-alpha.1) of hyper 0.13. [hyper](https://hyper.rs) is a maturing HTTP library written in Rust, already one of the fastest out there, and trusted by many for its correctness.

This alpha release brings support for the new [`std::future::Future`](https://doc.rust-lang.org/std/future/trait.Future.html). The reason this is so exciting is that this allows using the new `async`/`await` syntax that will be stabilizing in Rust 1.39.

### Example

The follow example shows how one can use `async`/`await` to dump a response to the console:

    #[tokio::main]
    async fn main() -> Result<(), Error> {
        let client = Client::new();
    
        let resp = client.get("http://httpbin.org/ip".parse()?).await?;
        println!("Status: {}", resp.status());
        println!("Headers: {:#?}\n", resp.headers());
    
        while let Some(chunk) = resp.body_mut().next().await {
            stdout().write_all(&chunk?)?;
        }
    
        Ok(())
    }

The same `async`/`await` style can be used for writing servers as well!

### Changes to come

Besides the change from `futures` 0.1 to `std::future::Future` and how writing code with `async`/`await`, much of hyper’s API will feel very similar. Still, there a some technically breaking changes that will be included in the 0.13 as well.

### Embracing `tower::Service`

During hyper 0.12, servers were defined via the `hyper::service::Service` trait. Since then, we’ve been working hard on a general `Service` interface, and building [powerful middleware](https://github.com/tower-rs/tower) that can utilize it. Our hope is that eventually, applications can be generic over [`Service`](https://docs.rs/tower-service/0.3.0-alpha.1/tower_service/trait.Service.html) and the `http` types, and a user could choose their backend that plugs right in (such as hyper).

Consider a small example that handles many mundane things for you:

    let svc = ServiceBuilder::new()
        // Reject the request early if concurrency limit is hit
        .load_shed()
        // Only allow 1,000 requests in flight at a time
        .concurrency_limit(1_000)
        // Cancel requests that hang too long
        .timeout(Duration::from_secs(30))
        // All wrapped around your application logic
        .service(your_app_service);

The `tower::Service` trait easily allows everyone to power up their servers and clients!

### Alpha One

This first alpha is to allow people to try out writing HTTP servers and clients using the new `async`/`await` syntax. All the features from 0.12 work in this release. However, not all the API changes have been finalized, so as with other alphas, there will likely be breakage between alpha releases as we fine tune things.

But for now, get your fresh copy of hyper [v0.13.0-alpha.1 here](https://github.com/hyperium/hyper/releases/tag/v0.13.0-alpha.1)!

    [dependencies]
    hyper = "=0.13.0-alpha.1"

