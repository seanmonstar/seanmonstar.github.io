---
layout: post
title: 'This Month in hyper: July 2022'
date: '2022-07-28T12:51:13-04:00'
tags:
- rust
- rust-lang
- hyper
- http
- curl
- monthly
- http3
tumblr_url: https://seanmonstar.com/post/691039972381294592/this-month-in-hyper-july-2022
---
I’d like to start providing more frequent updates of all the work that’s done in [hyper](https://hyper.rs), along with examples, details, and thanks. As a bonus, I included things from June in here as well.

### Release v0.14.20

While we will release often when there’s new bug fixes grouped up, [this release](https://github.com/hyperium/hyper/releases/tag/v0.14.20) has a helpful new feature for people who write proxies and gateways.

#### hyper::ext::ReasonPhrase

One of the new features in v0.14.20 is the ability to interact more with the HTTP/1 [reason-phrase](https://github.com/hyperium/hyper/pull/2792) of an HTTP response. This is a sort-of deprecated part of HTTP (the reason-phrase does not exist in HTTP/2 or 3), but sometimes important information is still put in them. The `http::StatusCode` type does not include a `String` field for the reason-phrase, because of that deprecation, and to decrease the memory size of the type.

Consider this response with a custom reason-phrase:

    HTTP/1.1 200 Stupendous

If you _do_ need to access it, now you can! If the client receives a response with a non-standard phrase, it will be saved in the response extensions. So, a client would do so like this:

    use hyper::ext::ReasonPhrase;
    
    let resp = client.get(url).await?;
    if let Some(phrase) = resp.extensions().get::<ReasonPhrase>() {
        eprintln!("got {} {:?}", resp.status(), phrase);
    }

And a server can _send_ a custom phrase by including the extension, like so:

    use hyper::ext::ReasonPhrase;
    
    async fn handle(_req: Request) -> Result<Response, Infallible> {
        let mut resp = Response::new(Body::empty());
        // There's also TryFrom impls for non-static values.
        let phrase = ReasonPhrase::from_static("Stupendous");
        resp.extensions_mut().insert(phrase);
        Ok(resp)
    }

Thanks a bunch to [Adam Foltzer](https://hyper.rs/blog/2022/04/08/welcome-adam-foltzer/) for the work on this feature!

### hyper 1.0 started

Development work on 1.0 started in June. Progress can be tracked in [the public dashboard](https://github.com/orgs/hyperium/projects/1/views/2). Some major work done so far includes splitting the client connection type per HTTP version, for forwards compatibility, and moving the connection pool implementation and the Tokio `connect` integration to `hyper-util`. Thanks to @oddgrd, @tomarkw, @dswij!

There’s several more big PRs nearing completion too, looking forward to thanking them next month.

### hyper in curl

In June, we fixed the way curl would pause and resume request bodies when using hyper, resulting in 4 more of the remaining unit tests to pass. In July, a brand new contributor (thanks @deantvv!) add the FFI function to use hyper’s existing obs-folded headers support. A PR just merged aligns how curl expects obs-folded headers and how hyper treats them, which should fix another couple unit tests.

There’s a [dashboard](https://github.com/orgs/hyperium/projects/2/views/1) if you’d like to [help this momentous work](https://seanmonstar.com/blog/2022-03-16-help-stabilize-hyper-in-curl/).

### HTTP/3

The HTTP/3 specification was published as [RFC 9114](https://httpwg.org/specs/rfc9114.html). I gave [a small recap](https://twitter.com/seanmonstar/status/1534224846015451136) of the state of HTTP/3 in hyper.

In the same time period, hyper’s [HTTP/3 library](https://github.com/hyperium/h3) saw some increased development, from three brand new contributors, two of which have made over 3 contributions each. The work includes making request streams splittable into a send and receive side, and support for “GREASE” streams and frames. Thank you @Ruben2424, @yu-re-ka, and @inflation!

I’ve started some work on a `tower-h3` crate, which would make it easier to plug HTTP/3 into an existing [Tower](https://github.com/tower-rs/tower) codebase. I really do need to push this stuff to GitHub.

I also wrote up an issue to [add experimental support to `reqwest`](https://github.com/seanmonstar/reqwest/issues/1558). Getting it into reqwest would mean users could start trying it out more easily, and we could learn how to adjust the `h3` crate API before making it part of `hyper` directly. It would likely be easier to do this if the `tower-h3` crate were ready, as an intermediate step. [Volunteers welcome!](https://discord.gg/q5mVhMD)

