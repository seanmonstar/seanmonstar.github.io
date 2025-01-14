---
layout: post
title: 'hyper in curl Needs a Champion'
date: '2024-11-19T09:48:00'
tags:
- rust
- hyper
- curl
- memory-safety
---

**tl;dr** - hyper in curl is nearly complete, but it needs a champion. Without a partner actively engaged that wants to enable and ship, it's now on the path for being deprecated and removed.

It needs a champion, a backing vendor or distro. Will that be you?

**UPDATE 2024-12-21**: [curl has dropped hyper][bye].

### Why would you put a hyper in a curl?

Why would you? Memory safety.

[Company](https://www.zdnet.com/article/microsoft-70-percent-of-all-security-bugs-are-memory-safety-issues/) after [company](https://www.chromium.org/Home/chromium-security/memory-safety), [product](https://security.googleblog.com/2019/05/queue-hardening-enhancements.html) after [product](https://hacks.mozilla.org/2019/02/rewriting-a-browser-component-in-rust/),  [report](https://langui.sh/2019/07/23/apple-memory-safety/) after [report](https://googleprojectzero.blogspot.com/p/0day.html). Memory _un_-safety. Causes. Serious. Issues.

[curl][] is _everywhere_. Billions of installations. It's likely all humans accessing the internet use curl.

A self-analysis of curl finds that [half of curl's vulnerabilities are C mistakes](https://daniel.haxx.se/blog/2021/03/09/half-of-curls-vulnerabilities-are-c-mistakes/). Memory safety.

As I said when [when we told the world about it](https://aws.amazon.com/blogs/opensource/how-using-hyper-in-curl-can-help-make-the-internet-safer/):

> Considering how much curl is used, this was an opportunity to make the _internet_ safer.

[hyper][] is the most mature HTTP library written in Rust. By making hyper a possible HTTP backend for curl, the code used for the most ubiquituous protocol could be made safer. Certainly true for HTTP/1, even more so with the much bigger (code-wise) HTTP/2 and HTTP/3.

So, why do this? Oh, right. Ahem. **Memory safety**.


### Most of the work is done

Let me back up a little.

In 2020, we started [exploring the idea](https://www.memorysafety.org/blog/memory-safe-curl/). I designed and built a [C API for hyper](https://docs.rs/hyper/latest/hyper/ffi/). Daniel refactored curl to allow for HTTP backends, and [integrated hyper](https://daniel.haxx.se/blog/2021/05/28/taking-hyper-curl-further/).

We got it nearly complete. Adventurous tinkerers were able to build and use it on their personal machines. Over 95% of curl's large test suite was passing.

I gave [a talk for curl up 2022](https://seanmonstar.com/blog/curl-up-2022-hyper-in-curl/) about the progress.[^since]

We're ready to finish, technically.

### Over the finish line

[Funding for an engineer](https://www.memorysafety.org/blog/memory-safe-curl/) to complete the work is available.

But the upkeep of the feature isn't free, in both the curl and hyper repositories. Because of that, and without a commited organization wanting to ship it, it's [planned to be removed](https://github.com/curl/curl/blob/7b12c36ca972d9e9a14088cdd88232385e619d44/docs/DEPRECATE.md#Hyper) at the start of 2025.

So, what exactly could change that? What is needed?

### Champion required

A champion, if you want it.[^want]

**A backing vendor or distro** that wants to enable and actively use the backend. A launch partner. Many people know what it's like to work on a large new feature, ask people to try it out, and everyone is too busy, assuming someone _else_ will. A launch partner actively tests it and provides feedback. 

There's [more incentive](https://thenewstack.io/feds-critical-software-must-drop-c-c-by-2026-or-face-risk/) to partner than ever, as we see companies [successfully make code safer](https://security.googleblog.com/2024/10/safer-with-google-advancing-memory.html). And this project is so close, adopting now can have a large impact compared to the remaining effort.

[Reach out to me][contact] if you want this to happen. Sooner rather than later. Let's make the _internet_ safer!

**UPDATE 2024-12-21**: [curl has dropped hyper][bye].

[^since]: Recently, a few more things have been improved on hyper's side. For instance, @nnethercote and @jsha [significantly improved](https://github.com/hyperium/hyper/pull/3296) [the C docs](https://github.com/hyperium/hyper/pull/3424), and  @hjr3 [added HTTP/1.1 trailers support](https://github.com/hyperium/hyper/pull/3637) that curl needed.

[^want]: People _always_ end up doing exactly what they want. There's a loud rewrite-it-in-Rust sub-community. Here's an opportunity. Actions show what people actually want.

[hyper]: https://hyper.rs
[curl]: https://curl.se
[contact]: https://seanmonstar.com/about#contact
[bye]: https://daniel.haxx.se/blog/2024/12/21/dropping-hyper/

