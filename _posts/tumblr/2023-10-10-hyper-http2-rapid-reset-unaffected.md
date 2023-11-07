---
layout: post
title: 'hyper HTTP/2 Rapid Reset Attack: Unaffected'
date: '2023-10-10T08:07-07:00'
tags:
- rust
- hyper
- http2
- security
tumblr_url: https://seanmonstar.com/post/730794151136935936/hyper-http2-rapid-reset-unaffected
---
Today, the world has been made aware of a potential vulnerability affecting most HTTP/2 implementations, sending a rapid amount of streams and resets.

If you use [hyper][], even just it's `h2` dependency, you are safe. **hyper is not affected**. Especially if you have `h2` v0.3.18 or newer. We manually verified that an example hyper server responds correctly. Big thanks to [@Noah-Kennedy](https://github.com/Noah-Kennedy) for all the help.

If you want to read more, checkout [CVE-2023-44487][], or these [other][google] [breakdowns][cf].

That's it!

You're still here. You want to know the "why"?

Well, for two main reasons.

We added in specific detection of [this problem back in April][coe]. A related flaw was reported against hyper, with the added requirement of a consistently flooded network. We fixed that. It had a CVE and RUSTSEC advisory for it, so you should have upgraded, right?

But even without that fix, the damage that could be done was local. The bigger concern of this newly announced vulnerability seems to be when the receipt of the `HEADERS` frame triggers more work in the handlers that needs to then be canceled. The way hyper handles frames, it will cancel out the stream before ever making it available for handlers, so the cost is local. Without the fix, and only if the user can flood the network, then hyper could consume a lot of memory keeping track of all the suddenly reset streams. If they can't flood the network, then no problem at all.

So if you've upgraded since April, you're safe. By the way...

Handling security by dealing with reports, and working with coordinated disclosures like today are a significant part of maintaining hyper. **If you appreciate that hyper is kept secure**, consider [sponsoring][sponsor]. Being able to have more support during security disclosures is something that you can setup with me privately.

[hyper]: https://hyper.rs
[CVE-2023-44487]: https://nvd.nist.gov/vuln/detail/CVE-2023-44487
[google]: https://cloud.google.com/blog/products/identity-security/how-it-works-the-novel-http2-rapid-reset-ddos-attack
[cf]: https://blog.cloudflare.com/technical-breakdown-http2-rapid-reset-ddos-attack/
[coe]: https://seanmonstar.com/post/715784167270596608/coe-surpise-hyper-cve
[sponsor]: https://seanmonstar.com/sponsor
