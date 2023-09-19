---
layout: post
title: hyper v0.6.8 - Socket Timeouts
date: '2015-08-03T20:36:25-04:00'
tags:
- rust
- rust-lang
- hyper
tumblr_url: https://seanmonstar.com/post/125800688202/hyper-v068-socket-timeouts
---
[hyper v0.6.8 - Socket Timeouts](https://github.com/hyperium/hyper/releases/tag/v0.6.8)  

The newest release of hyper includes an unstable feature to set read and write timeouts for those of your daring enough to live on nightly. To do so, you’ll need to enable the `timeouts` cargo feature:

    [dependencies.hyper]
    version = "0.6.8"
    features = "timeouts"

And then you can set them on a `Client` or `Server`:

    let mut client = Client::new();
    client.set_read_timeout(Some(Duration::from_secs(30));
    client.set_write_timeout(Some(Duration::from_secs(30));
    
    // ... or a server
    let mut server = Server::http(addr).unwrap();
    server.set_read_timeout(Some(Duration::from_secs(10));
    server.set_write_timeout(Some(Duration::from_secs(10));

