---
layout: post
title: 'hyper HTTP/2 Continuation Flood'
date: '2024-04-03T14:11:00'
tags:
- rust
- hyper
- http2
- security
---

Patches are available for [`h2`][h2], v0.4.4 and v0.3.26, to harden against a newly disclosed HTTP/2 attack vector, mostly for servers. If you need help, reach out for [support][sponsor].

If you're curious about more, read on.

### What is the attack?

In HTTP/2, there are a bunch of frame types. Some of those frames are related to sending headers (or fields). Since there is a maximum frame size, in order to send more headers than fit, a peer can send `CONTINUATION` frames that indicate more headers for the message.

That mechanism is the source of a new attack, trying to send an infinite number of `CONTINUATION` frames to starve or kill a server. This was a coordinated release, with various implementations involved. [VU#421664][VU] has details about the others.

### hyper's `h2` is somewhat affected

The HTTP/2 specification doesn't state a default value for `SETTING_MAX_HEADER_LIST_SIZE`. You should set that to something reasonable for your use case. But even if you don't, `h2` picks an emergency default, which is fairly high so to as to not reject requests for users who need big requests.

Once a stream of headers and continuation frames reach the limit, either configured or the large default, memory growth will cap there. `h2` will keep reading and decoding, but discarding any new headers. Really, it's trying to keep the HPACK table in sync, and will send back a stream error once finished of "too big".

The problem is that `h2` won't stop reading as long each CONTINUATION frame says there are more coming. So the memory stays the same, but it will use CPU to read and discard. And since the headers are never "complete", the application doesn't get to know about the processing of frames.

However, if deployed on Tokio (or some other runtime with a similar feature), the [task budget][] will prevent it from locking out other tasks. Other requests _can_ be served, and responded to. But it will make it slower.

A degradation of service.[^cve]

### What we did

Besides testing and determining the above, we also worked on a fix.[^who]

We realized that while a lower `SETTING_MAX_HEADER_LIST` does help, it's not the only thing that needs to be checked. The reason for that is because a `CONTINUATION` frame could be sent with the smallest header value, and thus take longer to reach that maximum. That will still consume resources processing frames.

We figured that a maximum needed to be set on the allowed `CONTINUATION` frames. But a naïve hard-coded number has its own problems.

The maximum frame size isn't usually changed, which means you're stuck with 16kb frames. And if you _do_ want to allow message headers bigger than that, then you need to allow `CONTINUATION` frames. Since both frame size and header list size are configurable, a hard-coded limit wouldn't work.

We settled on calculating how many `CONTINUATION` frames would be needed for a legitimate message to send up to the max header list size. We also add a little bit of padding to that number, to account for frames that may not be perfectly packed.

The patch has been tried out on some production servers, and found no false positives.

### What you can do

- If you don't use HTTP/2 on a server, then don't worry.
    - If you do, but are not exposed to untrusted L7 traffic, also don't worry.
- If you don't set the `SETTING_MAX_HEADER_LIST_SIZE`, you should consider doing that.
- You should run `cargo update -p h2` in your application to pull in the latest versions with fixes (v0.4.4 and v0.3.26).

If you need help, reach out for [support][sponsor]. Or send [thanks][sponsor], so that months long security work like this can be done in the future.

[^cve]: For this reason, we didn't create a CVE. I'm sure some well tell me I'm wrong. I'm already skeptical of CVEs for denial-of-service; it's a big deal for some people, and not at all for others. I'm also very wary of alert fatigue. A _degradation_ of service, where things are just slower, does not feel like a "wake-up-your-team" moment. (We did submit a RustSec advisory to nudge people to upgrade.)

[^who]: [Noah Kennedy][] and I have worked on this on and off for a few months, while waiting for the VINCE deadline. Thanks Noah!

[h2]: https://crates.io/crates/h2
[sponsor]: https://seanmonstar.com/sponsor
[task budget]: https://tokio.rs/blog/2020-04-preemption
[Noah Kennedy]: https://hyper.rs/blog/2024/02/28/welcome-noah-kennedy/
[VU]: https://www.kb.cert.org/vuls/id/421644

