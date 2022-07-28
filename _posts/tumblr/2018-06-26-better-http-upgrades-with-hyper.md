---
layout: post
title: Better HTTP Upgrades with hyper
date: '2018-06-26T12:39:42-07:00'
tags:
- hyper
- rust
- rust-lang
- http
- websockets
tumblr_url: https://seanmonstar.com/post/175280388657/better-http-upgrades-with-hyper
---
It’s been possible to [handle HTTP Upgrades](http://seanmonstar.com/post/172531530657/http-upgrades-with-hyper) (like Websockets) in [hyper](https://hyper.rs) if you made use of the low-level APIs in the server and client, but it wasn’t especially nice to work with. It also meant to handle upgrades, you couldn’t use the nicer things that hyper takes care of for you with `Client` or `Server`.

In hyper [v0.12.3](https://github.com/hyperium/hyper/releases/tag/v0.12.3)<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>, **handling upgrades is much easier!**

### Body::on\_upgrade()

The mechanism for handling upgrades and `CONNECT` is unified into a `Future` on the `hyper::Body` type. The way this works is in either case, `Client` or `Server`, you’re already receiving a `hyper::Body` that represents the streamed body from the remote. It also happens to be a great place to store a flag of whether an HTTP Upgrade is possible.

For now, after using a `Body` to get any data, you can convert it into a `Future` that yields the “upgraded” connection on success. With lessons learned from the lower-level upgrade process, the returned `Upgraded` opts for a default of easier-to-use. It implements `Read` and `Write`, and those implementations will check the read buffer for any previously read bytes before the upgrade completed. The easiest thing to do is just to treat the yielded `Upgraded` type as some `impl Read + Write`, and use it as such for the next protocol you plan to use.

In order to provide this API, the `Upgraded` holds the IO type as a boxed trait object internally. If dynamic dispatch is undesirable, there is `Upgraded::downcast` to try to convert into the original type, along with the remaining read buffer.

Take a look at these simplified examples upgrading to Websockets:

### Client Upgrades

    let client = Client::new();
    
    let req = Request::builder()
        .uri("http://example.local/chat")
        .header("upgrade", "websocket")
        .header("connection", "upgrade")
        .body(Body::empty()
        .unwrap();
    
    // This builds a future that should be spawned on an executor...
    client
        .request(req)
        .and_then(|res| {
            res.into_body().on_upgrade()
        })
        .and_then(|upgraded| {
            // just use this as an IO
            websocket_lib::client_thing(upgraded)
        })

### Server Upgrades

    let service = service_fn_ok(|req| {
        // Just assuming its always an upgrade for this example...
    
        let upgrade = req
            .into_body()
            .on_upgrade()
            .map(|upgraded| {
                websocket_lib::server_thing(upgraded);
            })
            .map_err(|err| eprintln!("upgrade error: {}", err));
    
        hyper::rt::spawn(upgrade);
    
        Response::builder()
            .status(101)
            .header("upgrade", "websocket")
            .header("connection", "upgrade")
            .body(Body::empty())
            .unwrap()
    });

There’s a [fuller example](https://github.com/hyperium/hyper/blob/master/examples/upgrades.rs) of a client and server that upgrade in the same program as well.

* * *

1. 

Most support was made available in [v0.12.2](https://github.com/hyperium/hyper/releases/tag/v0.12.2), but [v0.12.3](https://github.com/hyperium/hyper/releases/tag/v0.12.3) fixed a couple missing pieces when trying to do `CONNECT` requests over the `Client`. Everything else worked in [v0.12.2](https://github.com/hyperium/hyper/releases/tag/v0.12.2).&nbsp;[↩︎](#fnref:1)

