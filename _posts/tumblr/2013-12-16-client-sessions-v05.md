---
layout: post
title: client-sessions v0.5
date: '2013-12-16T11:11:30-08:00'
tags:
- javascript
- programming
- nodejs
- client-sessions
- planet
tumblr_url: https://seanmonstar.com/post/70209872571/client-sessions-v05
---
[client-sessions v0.5](https://github.com/mozilla/node-client-sessions/releases/tag/v0.5.0)  

It rather bothered me that [client-sessions](https://npmjs.org/package/client-sessions) required a native module, but it used a Proxy so as to only re-encrypt the cookie if it a property changed. Stuck 30,000 feet in the air unnaturally, I spent some time removing the usage of the Proxy, and use getters and setters instead.

I also mopped up some bugs while I was at it.

So, [v0.5](https://github.com/mozilla/node-client-sessions/releases/tag/v0.5.0) is live, works better, and such!

