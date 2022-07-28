---
layout: post
title: hyper on beta
date: '2015-04-08T12:33:28-07:00'
tags:
- rust
- hyper
- mozilla
- planet
- http
- rust-lang
tumblr_url: https://seanmonstar.com/post/115873169212/hyper-on-beta
---
Since I [announced hyper](http://seanmonstar.com/2022/07/28/2014-12-18-hyper.html) in December of last year, it has continued to grow as Rust’s http library.

### highlights

- A [changelog](https://github.com/hyperium/hyper/blob/master/CHANGELOG.md) was added!<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>
- Many more of the common headers have been implemented.
- [Pyfisch has been slaving away](https://github.com/hyperium/hyper/commits/master?author=pyfisch) at cleaning up the headers modules.
- A switch to using [httparse](https://github.com/hyperium/hyper/commit/b87bb20f0c25891c30ef2399da2721596fbc1fcf) for HTTP/1.x parsing with wonderful speed gains.
- Reem wrote up a [faster threadpool](https://github.com/hyperium/hyper/commit/3528fb9b015a0959268452d5b42d5544c7b98a6a) for the Server.
- **We compile on rustc beta!**

Of course, a lot of the effort this past few months has been keeping up with all the changes to Rust and the standard library. A million thanks to [all those who helped](https://github.com/hyperium/hyper/graphs/contributors) with these upgrades. I can’t overstate the joy it is to wake up in the morning, read that there are breaking changes in the latest nightly, and then see in my inbox that a pull request has already been filed fixing hyper.

### up next

Now that the breaking changes are behind us, developement can focus entirely on making hyper do things better-er. Specifically, here’s things that are either in progress, or highly desired (hint hint).

- The Client is very close to receiving [Connection Pool](https://github.com/hyperium/hyper/issues/363) support. Code is in a branch.
- The Server is switching to a request queue, such that keep-alive connections don’t [starve the server](https://github.com/hyperium/hyper/issues/368).
- [Marko Lalic has written an HTTP/2 library in Rust](https://mlalic.blogspot.com/2015/03/solicit-http2-library-for-rust.html). Many have stated wanting to help [integrate it into hyper](https://github.com/hyperium/hyper/issues/304). Hopefully, this should happen soon!
- We’d like to switch to [asynchronous IO](https://github.com/hyperium/hyper/issues/395). There’s been plenty of working happening in [mio](https://github.com/carllerche/mio), which brings us async IO for Unix-y platforms. There’s need for a Windows async IO library, and a library that can wrap the two. Then we can start switching over, and that means more speeeed.

I could imagine aiming for a 1.0 of hyper once we have asynchronous IO.

Again, all of this is [thanks to you guys](https://github.com/hyperium/hyper/graphs/contributors), the amazing community. And if you want to get involved, please join in. Perhaps try tackling one of the [easy](https://github.com/hyperium/hyper/labels/easy) issues first.

* * *

1. 

Or you can check the [Releases](https://github.com/hyperium/hyper/releases). I try to keep them in sync.&nbsp;[↩︎](#fnref:1)

