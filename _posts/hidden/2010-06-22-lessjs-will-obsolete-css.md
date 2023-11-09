---
hidden: true
layout: post
title: Less.js Will Obsolete CSS
date: '2010-06-22T15:30:00-04:00'
tags:
- css
- javascript
tumblr_url: https://seanmonstar.com/post/726287616/lessjs-will-obsolete-css
---
[Less.js Will Obsolete CSS](http://fadeyev.net/2010/06/19/lessjs-will-obsolete-css/)  

I’m starting to really like the idea of writing _less_ CSS. Especially with all the browser-prefixed properties. It’d be nice to just write one `border-radius` and have a processor expand it for me.

Less looks promising. Though I’m hesitant about having the processor execute on the client side. It does cache itself in `localStorage` if the browser supports it. But what about ones that don’t (all of IE)? IE doesn’t have a blazing fast JavaScript interpreter to begin with.

**Plus** : Every single visitor must spend CPU cycles processing the LessCSS. When the exact same output will be reached every time, I would rather just do it once server side and then serve up a static CSS file. Processed once, cached by _all_ browsers.

