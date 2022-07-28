---
layout: post
title: Introducing Reqwest
date: '2016-11-15T09:26:14-08:00'
tags:
- rust
- rust-lang
- reqwest
- mozilla
- planet
- programming
tumblr_url: https://seanmonstar.com/post/153221119046/introducing-reqwest
---
In web development, you can find resource after resource, framework after library, all helping you to build a web server. But what if you need to use a client? So many applications need to download something from the web, or to upload some data, and in many instances are left with the standard HTTP library to do it. That works, but it’s so much better to be able to reach for a tool that includes the batteries in the packaging. Several languages have something like this. Now Rust does too.

If you need to make HTTP requests in your application, you probably want to reach for [reqwest](https://github.com/seanmonstar/reqwest).<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>

### Convenience

There are several parts of HTTP that we usually just want to happen for us automatically. For many of us, these extras are not something we’d consider ‘extra’, but just business-as-usual. This includes things like following redirects, connection pooling (keep-alive), JSON payloads, cookies, and more.

The 0.1 release includes some of these things already, and the missing parts should come in subsequent releases. To fetch a particular URL, it can be as simple as this:

    let res = reqwest::get("https://rust-lang.org")?;

From there, maybe you just want to dump the page into the console:

    ::std::io::copy(&mut res, &mut ::std::io::stdout())?;

What about sending bodies in a `POST` request?

#### Bodies

You could send the raw bytes of anything you want using the `body()` method of a `RequestBuilder`. But more likely, there are a couple of formats that are far more common that we just want to taken care of for us. 0.1 provides convenience methods for sending forms (urlencoded) and JSON data.

    let client = reqwest::Client::new()?;
    let res = client.post("https://httpbin.org/post")
        .form(&[("foo", "bar"), ("baz", "quux")])
        .send()?;

Or you could build a `HashMap` and send that as a form instead. Indeed, the `form()` and `json()` methods take any value that implements [`Serialize`](http://serde.rs), so you could even use a custom struct.

    #[derive(Serialize)]
    struct User {
        name: String,
        id: u64,
    }
    
    let user = User {
        name: String::from("Sean"),
        id: 42,
    };
    
    let res = client.post("https://httpbin.org/post")
        .json(&user)
        .send()?;

Easing the sending of [multipart](https://github.com/seanmonstar/reqwest/issues/4) forms is a feature that will hopefully be added shortly in 0.2.

### TLS

The way the `reqwest` crate handles TLS is similar to cURL. It uses the awesome new [native-tls](https://github.com/sfackler/rust-native-tls) crate to make use of built-in-to-the-OS TLS implementations when they exist, and using the new OpenSSL 1.1 if it does not. For now, that means it will use `security-framework` on masOS, and `schannel` on Windows. It’s plausible that something like [rustls](https://github.com/ctz/rustls) (or ring or something else) would eventually replace the OpenSSL backup in the future.

Blah blah, what does all that mean for you? That on whichever OS you happen to be using `reqwest`, it will try to provide the easiest experience for you when connecting to HTTPS websites.

### The Future

Another reason to use `reqwest` is to ease the upcoming changes to [hyper](http://hyper.rs), which is adopting non-blocking IO. Many applications do not need to make thousands of requests. Many just need to make 1, or a few, and writing code in a blocking style is easier. Without the need to make thousands of requests, the benefits of non-blocking IO are fewer. So, `reqwest::Client` plans to always provide a blocking API. Even when hyper releases with non-blocking IO, `reqwest` will upgrade to it and still present the `Client` with the same API. Your code won’t need to change, but it will become more robust underneath.<sup id="fnref:2"><a href="#fn:2" class="footnote-ref" role="doc-noteref">2</a></sup>

Others will likely want many of the convenient features of `reqwest`, but _with_ non-blocking sockets instead. There will likely be a `reqwest::AsyncClient` or similar added as well.

Besides the eventual upgrade to non-blocking IO, the plan is that `reqwest` will gain other conveniences as well. Currently proposed ideas include:

- Cookie Jar support
- Customizable redirect policies
- Automatic gzip and brotli decoding
- Proxies
- Multipart payloads

[Come help make it happen!](https://github.com/seanmonstar/reqwest)

* * *

1. 

The odd name is an unfortunate consequence of being late to the party. The `request` crate is effectively abandonware. I’ve tried reaching out to the author in various ways, but he seems to have disappeared from the internets. The `requests` crate (with an ’s’) also exists, but does seem to be actively developed.&nbsp;[↩︎](#fnref:1)

2. 

Using non-blocking sockets internally is still an improvement for anyone wanting a blocking API. It’s not currently possible to make the blocking TCP sockets have a timeout when doing DNS resolution and connecting, but asynchronous versions can use a timeout. It also means it is easier to determine when a socket in the pool has been closed, and can recycle it more reliably.&nbsp;[↩︎](#fnref:2)

