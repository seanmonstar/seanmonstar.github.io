---
layout: post
title: "hyper-util Composable Pools"
excerpt: "Announcing connection pool layers allowing advanced composition in Rust."
date: '2025-12-03T08:48:00'
tags:
- rust
- hyper
- hyper-util
- tower
- open-source
- programming
- bestof
---
I'm so excited to announce [hyper][]'s new composable pool layers![^excited]

As part of [making reqwest more modular](https://seanmonstar.com/blog/modular-reqwest/), we've designed a new connection pool, and made the pieces available in `hyper_util::client::pool`. But this is more than just a "hey, we have a Pool, it moved other there." We've literally pulled apart the pool, in a way I haven't found elsewhere.

Building a purpose‑specific pool is now straightforward. Add the features you want, even custom ones, and skip the bloat, no forks required.

Read on to see what exactly we solved, how, and what comes next. If you just want to use them, [here's the docs][docs]. Everyone else, let's dive in.

### We started with the users

We started with the users, looking back over past issues filed, common questions in chat, and private conversations explaining what they needed to do. Boiled down, that got us to these requirements:

- A full-featured pool, like the one in [`legacy`][], must be possible.
- Microservices shouldn't have to handle multiple protocols or hostnames.
- Some clients need custom keys for the pool.
- Others need to limit new connections made at a time.
- Or cap the total number of connections.
- Customize connection expiration based on idle time, max lifetime, or even [poisoning][].
- And importantly, allow custom logic not already thought of.
    
From past experience combining middleware, I had a strong feeling the pool requirements could be broken up into `tower` layers. But what would that even _look_ like? Would it be horrible to use?

To answer that, we took the requirements and considered the developer experience of using layers. It had to feel nice. Not just to write, but also to come back to and read.

I then sketched out several of these layers to make sure they could actually work. Once most of it was working, the [proposal][] was ready.


### The initial 4 working pools

No plan survives contact with the enemy. We originally proposed five pool types, but launch with just the following four: singleton, cache, negotiate, map.

The `singleton` pool wraps a connector[^mksvc] that should only produce a single active connection. It bundles all concurrent calls so only one connection is made. All calls to the singleton will return a clone of the inner service once established. This fits the HTTP/2 case well.

The `cache` pool maintains a list of cached services produced by a connector. Calling the cache returns either an existing service, or makes a new one. When dropped, the cached service is returned to the cache if possible. Importantly for performance, the cache supports connection racing, just like the legacy pool.

The `negotiate` pool allows for a service that can decide between two service types based on an intermediate return value. Unlike typical routing, it makes decisions based on the response (the connection) rather than the request. The main use case is supporting ALPN upgrades to HTTP/2, with a fallback to HTTP/1. And its design allows combining two different pooling strategies.

The `map` pool isn't a typical service like the other pools, but rather is a stand-alone type that maps requests to keys and connectors. As a kind of router, it cannot determine which inner service to check for backpressure until the request is made. The map implementation allows customization of extracting a key, and how to construct a connector for that key.

### Ineffably unstable

I knew this work would land in `hyper-util` first, because it's not stable yet. Being so freshly designed, changes are expected after some more real-world usage. Still, I wanted to shield earlier adopters from breaking changes. At the same time, valuing performance and flexibility, I wanted to push as much as reasonably possible into the type system.

When initially tinkering during the summer, I had one of _those_ thoughts. The kind that clangs like a giant lock snapping open: what about type-state builders and unnameable types? I took a side quest, and tackled the [warp v0.4 upgrade](https://seanmonstar.com/blog/warp-v04/), to test out this API design. That post explains it a bit more.

The various threads were all coming together.

With each pool concept a `tower` service, once composed, a user shouldn't care what it is beyond being some `impl Service`. I tested this out in `reqwest`, and yea, I don't need to name the types. While I did need _a_ type, I was able to store a `dyn Service`, and inference handled the rest.

### Real world usage: in reqwest

Once those main pieces seemed ready, I needed a real example to test drive them. Tool-makers that don't use their tools make bad tools, after all.

I started by replacing the `legacy` pool inside `reqwest`. Part of the larger diff in reqwest is handling all of reqwest's different pool configuration options.

But, putting the default case together is pretty self-explanatory:

```rust
// Note: some noise has been trimmed
let http1 = (
    pool::cache(exec),
    util::http1_request_target(),
    util::http1_set_host(),
    util::meta(MyMetaIdleAt::new),
    conn::http1(),
);

let http2 = (
    pool::singleton(),
    conn::http2(),
);

let pool_layers = tower::layer::layer_fn(move |svc| {
    pool::negotiate::builder()
        .fallback(http1.clone())
        .upgrade(http2.clone())
        .inspect(|conn| conn.is_negotiated_h2())
        .connect(svc)
        .build()
});

let pool_map = pool::map::builder::<http::Uri>()
    .keys(|dst| scheme_and_auth(dst))
    .values(move |_dst| {
        pool_layers.layer(connector.clone())
    })
    .build();
```

And it works! Making the full-featured pool was one of the requirements: check. But, the next part was even more important.

As I mentioned before, I punted one of the proposed types: `expire`. Expiration is a necessary concept to a pool. But try as I might to fit the various generic shapes, it just wasn't happening. Thankfully, this work had a hard deadline. And deadlines keep you user-driven: let them have _something_ now, it can always be better later.

To prove the general design allowed expiration, I implemented a specific version of it directly in reqwest.

```rust
tokio::spawn(async move {
    loop {
        tokio::time::sleep(idle_dur).await;
        let now = Instant::now();
        let Some(pool) = pool_map.upgrade() else { return };
        
        pool.lock().unwrap().retain(|_key, svc| {
            svc.fallback_mut().retain(|svc| {
                if svc.inner().inner().inner().is_closed() {
                    return false;
                }

                if let Some(idle_at) = svc.meta().idle_at {
                    return now > idle_at + idle_dur;
                }
                true
            });
            svc.upgrade_mut().retain(|svc| {
                !svc.is_closed()
            });
            !svc.fallback_mut().is_empty() || !svc.upgrade_mut().is_empty()
        });
    }
});
```
 
The ease of adding it helped solidify to me that this was definitely the right design. I was able to slot in a `meta` layer tracking idle time, and then use that to `retain` services. I placed that layer in right next to some of the other HTTP/1-specific layers. Easy!

### Being modular opens up customization

With the ability to build a stack for your pool, consider an example of how we can start to solve other requirements listed earlier.

```rust
let svc = ServiceBuilder::new()
    // cached connections are unaware of the limit
    .layer(pool::cache())
    // in-flight handshakes are limited
    .concurrency_limit(5)    
    .layer(conn::http1())
    .service(connect::tcp());
```

It also allows adding in layers we don't currently have, such as per-host connection semaphores, or a few layers up over all hosts. Adding new functionality isn't blocked on us, and no one has to "pay" for features they don't need.

I can't wait to see what else is done with the design!

### Pools ready

The `hyper_util::client::pool` module is now available in [v0.1.19][release]. Go check the [docs][], and try to build cool things. Please file issues if parts are missing, we'll keep iterating.

I've been working on this feature set for long time. It's something I started thinking about years ago, and after months of work this year, it feels awesome to finally be able to release it.

Thanks to my [sponsors][], retainers, and grants for making this all possible!


[^excited]: I mean, who isn't excited to announce anything? /s
[^mksvc]: All "connectors" are actually `MakeService`s, which are jsut a `Service` that produces a `Service`. It doesn't _have_ to create a connection, but it reads better when talking about pools.

[hyper]: https://hyper.rs
[proposal]: https://github.com/hyperium/hyper/issues/3948
[`legacy`]: https://docs.rs/hyper-util/0.1.x/hyper_util/client/legacy/struct.Client.html
[poisoning]: https://docs.rs/hyper-util/0.1.x/hyper_util/client/legacy/connect/struct.Connected.html#method.poison
[release]: https://github.com/hyperium/hyper-util/releases/tag/v0.1.19
[docs]: https://docs.rs/hyper-util/0.1.x/hyper_util/client/pool/
[sponsors]: https://seanmonstar.com/sponsor
