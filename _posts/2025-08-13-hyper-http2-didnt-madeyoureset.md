---
layout: post
title: "hyper HTTP/2 (Didn't) MadeYouReset"
date: '2025-08-13T08:25:00'
tags:
- rust
- hyper
- http2
- security
---
A new HTTP/2 attack vector was disclosed today called MadeYouReset. [hyper][]'s `h2` is negligably affected, weathering the attack well. But, we have provided patches just in case. We published patches weeks ago, so if you've been keeping up-to-date, you're fine! If not, you're most likely fine, but you can upgrade now. No CVE or security advisory is included with this.

This sort of work is supported **exclusively** through [retainers][retainer], get in contact to support.

Follow along for the background information, if you're curious!

### What is the attack

The attack is quite similar to the previous [Rapid Reset][] attack from a couple years ago. That is, it starts new requests up to the limit imposed by the server. And then instead of _sending_ explicit `RST_FRAMES`, it sends malformed frames that force the _server_ to reset the stream.

So far, the described behavior is normal HTTP/2 flow. The problem occurs in some implementations that don't respond well to quick cancelation.

As the attack describes, in some implementations it allows for more stream concurrency than might have been configured. See more in [VU#767506][VINCE].

### What hyper does

A couple years ago, we added a defense mechanism to `h2` that keeps count of how many times a frame from the remote causes a local reset, and once a configurable limit is reached, the connection is abruptly closed. This mechanism caught nearly all variants of the attack.

We had forgotten one code path dealing with `WINDOW_UPDATE` frames. But what happens when new streams are created and then a bad window frame is received?

In `h2`, the reset is sent to user code. Check the request body will notice the error. Additionally, `hyper`'s server code using `h2` is essentially waiting simultaneously on the user's service to reply, and for any reset to be received. Once the reset is received, the service future is canceled.[^react]

So, not much. When a simple hello world server example was tested against the attack, the only thing that happened was increased CPU usage. Which makes sense, it's receiving and canceling more streams.

**Stream concurrency is enforced**, since all futures are canceled immediately, and a signal is sent to anything listening for longer.

All of this before any additional patch. 

### What we did

We received a report[^ghsa] about the attack from security researchers[^researchers] on May 23, 2025. The attack includes many variants, so we set out trying to determine if hyper was vulnerable. We knew that hyper already added a defense mechanism for this exact sort of abuse. Only one of the variants did not get cut off, but saw increased CPU usage, so we dug in. It turns out the original feature forgot to check the limit when WINDOW_UPDATE frames were bad.

We determined that while the variants was not caught, it also did not break any guarantees. The increase in CPU usage was obvious, since it was necessarily handling more requests in general. Also important to our determination: the attack is _observable_ by user code, so any user code can react however they wish.

The researchers did a great job of explaining the concept behind the attack, providing a proof-of-concept script that we could test with, and conversed respectfully and thoughtfully as we went through the investigation.

Because the code of the aforementioned mechanism was already public, and the damage quite benign, we decided to release a patch weeks before the general embargo. It was just a refactor, converting it from logic-based to utilizing the type system more, so no other cases are "forgotten". That patch was released on June 30, 2025.

For similar reasons, we also determined there was no need for a CVE or security advisory.

The researchers came back a little afterwards showing that in some cases, servers using `h2` under this attack could be forced to crash. They even brough a new reproducible proof-of-concept. After analyzing it, we determined that the problem was not that of hyper/h2, but that of user code which might have unbounded queues and does not react to cancelation.

We decided to stick with our original determination.

### What you can do

After reading all that, what more can you do? Again, you're likely fine! But here's some good ideas, regardless:

- If you've been staying up-to-date, you're already done!
- If you have a server running hyper, it would be wise to make sure your server handlers really can cancel easily.
- You can run `cargo update -p h2` to upgrade to the latest versions (at least `0.4.11` and `0.3.27`).
- If you're running hyper v0.14 (so h2 v0.3.x), it's time to upgrade to v1. After nearly 2 years, patches likely won't be backported anymore.

There's one more thing you can consider: set up a [retainer][] to make sure security reports get taken care of. This was a considerable amount of important work. This was several months of higher-stress but important work which doesn't look like shipping a new feature.[^psirt]

Besides funding that important work, I'm also happy to provide pre-disclosure and hands-on guidance to those with a retainer.

[^react]: This difference from Rapid Reset is also important. In Rapid Reset, the resets were not observable by user code, so they could not react to the wasting of resources. With MadeYouReset, however, users can if they need to.
[^ghsa]: I just realized today that I cannot publish the GHSA as an informational, since it does not include a severity or CVE. GitHub, please?
[^researchers]: The original reporters were Gal Bar Nahum, Anat Bremler-Barr, and Yaniv Harel of Tel Aviv University. I primarily interacted with Gal, which was a 5/5 experience, would repeat.
[^psirt]: On top of what has been outlined here, there's a lot of coordination work involved, dealing with [VINCE][] and other vendors.

[hyper]: https://hyper.rs
[Rapid Reset]: https://seanmonstar.com/blog/hyper-http2-rapid-reset-unaffected/
[retainer]: https://seanmonstar.com/sponsor
[VINCE]: https://www.kb.cert.org/vuls/id/767506
