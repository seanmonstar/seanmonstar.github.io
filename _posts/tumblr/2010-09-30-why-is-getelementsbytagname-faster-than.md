---
layout: post
title: Why is getElementsByTagName() faster than querySelectorAll()?
date: '2010-09-30T10:40:00-07:00'
tags:
- javascript
- performance
- dom
tumblr_url: https://seanmonstar.com/post/1216429592/why-is-getelementsbytagname-faster-than
---
[Why is getElementsByTagName() faster than querySelectorAll()?](http://www.nczonline.net/blog/2010/09/28/why-is-getelementsbytagname-faster-that-queryselectorall/)  

Should come as no surprise if you know that `getElementsByTagName` returns a _live_ NodeList, which means its grabbing a cached item from the DOM without doing much work collecting nodes.

You pay the cost later if you do much looping on that live list.

