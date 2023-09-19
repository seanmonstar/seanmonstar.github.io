---
layout: post
title: client-sessions v0.4.0
date: '2013-11-05T14:25:36-05:00'
tags:
- javascript
- programming
- nodejs
- client-sessions
- mozilla
- planet
tumblr_url: https://seanmonstar.com/post/66111341154/client-sessions-v0-4-0
---
We released v0.4.0 of [client-sessions](https://npmjs.org/package/client-sessions) today, despite all the npm bumpiness. Here’s the [changelog](https://github.com/mozilla/node-client-sessions/releases/tag/v0.4.0):

- add activeDuration with default to 5 minutes
- add checking for native Proxy before using node-proxy
- add cookie.ephemeral option, default false
- add constant-time check
- adds self-aware check. wont override req.session if already exists
- fix wrong handled of utf8 replacement character
- fix http expiry of cookie to match duration
- fix updating cookie expiry whenever duration/createdAt changes
