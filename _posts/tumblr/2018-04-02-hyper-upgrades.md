---
layout: post
title: HTTP upgrades with hyper
date: '2018-04-02T14:34:53-07:00'
tags:
- hyper
- rust
- rust-lang
tumblr_url: https://seanmonstar.com/post/172531530657/hyper-upgrades
---
The [newest release](https://github.com/hyperium/hyper/releases/tag/v0.11.23) of [hyper](https://hyper.rs) includes some **lower-level connection APIs** for both the [server](server::conn) and [client](client::conn).

These allow finer-grained control of exactly what is happening during an HTTP request, and the higher-level APIs already provided by hyper use these internally. However, the point is that if control is what you need, these APIs allow you to build your own abstractions on top, such as a different implementation of connection pooling than the one provided in `hyper::Client`. Some examples of the exact control this gives include:

- The exact `Uri` to write, such as using the origin-form or absolute-form when talking to a proxy.
- The `Host` header.
- When to shutdown the connection (or to pool them).
- Take over the underlying IO transport again.

### A Websocket Upgrade

Notably, this allows using hyper send and receive **HTTP upgrade requests**. The most popular of these is Websockets. Here’s an example of how you could get Websockets working with hyper, broken up into 3 parts:

The first step is to create some IO transport, such as connecting a `TcpStream`, and then preparing an HTTP handshake.

    TcpStream::connect(&addr, &handle)
        .and_then(|tcp| {
            // preparing HTTP over this socket
            hyper::client::conn::handshake(tcp)
        })

The above is a `Future` that fulfills with a `SendRequest` and `Connection`. The `SendRequest` is used to, surprise, send requests over the connection. The `Connection` is a `Future` that should be polled continually to make progress on the HTTP state machine.

Once you have those, you could send a specific upgrade request, and prepare to get a `101 Switching Protocols` response.

    .and_then(|(mut client, conn)| {
        let req = Request::new(Method::Get, "/chat".parse().unwrap());
        // should set relevant Upgrade headers
    
        let res = client.send_request(req)
            .inspect(|res| {
                // Probably shouldn't actually use asserts
                assert_eq!(res.status(), StatusCode::SwitchingProtocols);
                assert_eq!(
                    res.headers().get_raw("Upgrade").unwrap(),
                    "websocket"
                );
            })
    
        // Put in an Option so poll_fn can return it later
        let mut conn = Some(conn);
        let until_upgrade = future::poll_fn(move || {
            try_ready!(conn
                .as_mut()
                .unwrap()
                // Poll until HTTP is complete, but don't shutdown socket
                .poll_without_shutdown()
            );
            Ok(Async::Ready(conn.take().unwrap())
        });
    
        res.join(until_upgrade)
    })

The above joins on a future of the `Response`, and a custom mini future that polls the `Connection` without trigger a full shutdown. The `Connection` will process its state until it notices that a 101 response was received (or the connection was closed normally (or with an error)). However, it _won’t_ call `shutdown` on the provided IO transport. This is key if you plan to continue using the transport after HTTP is completed.

Next, the connection can be pulled apart to get the “upgraded” transport back:

    .and_then(|(res, conn)| {
        // maybe get other headers from `res
    
        // Break up the HTTP connection into its parts
        let parts = conn.into_parts();
        let tcp = parts.io;
        // If the server sent the first message together with
        // the 101 response, it would be buffered here
        let buffered = parts.read_buf;
    
        // treat 'tcp' as a Websocket
    })

The principle is the same with the server, just with accepting connections and responding with the 101 response instead.

This works with upgrading to any protocol, and even `CONNECT` requests. An easier API to do this with a `hyper::Client` or `hyper::Server` is still in exploration, but the building block are there for anyone to do it exactly as they see fit.

## Moving on towards 0.12

This release is the last planned feature release for the 0.11.x branch of hyper. Progress towards the [0.12 feature set](https://github.com/hyperium/hyper/milestone/4) is now underway! Here’s a peak at some things that have already been accomplished on master:

- hyper has replaced its types with those from the `http` crate.
- Replaced tokio-core usage with the new tokio crate (huge thanks to [@srijs](https://github.com/srijs) for this and next point!).
- Upgraded to futures 0.2.
- Improvements to the `Connect` and `Body` APIs in preparation for better HTTP2 support.

If you’d like to join us in making this, take a peek at the [0.12 milestone](https://github.com/hyperium/hyper/milestone/4), or hop by the [#hyper IRC room](https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23hyper)!

