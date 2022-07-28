---
layout: post
title: 'HTML5: Changing the browser-URL without refreshing page'
date: '2010-07-20T17:18:03-07:00'
tags:
- javascript
tumblr_url: https://seanmonstar.com/post/838416615/new-history-pushstate
---
[HTML5: Changing the browser-URL without refreshing page](http://www.spoiledmilk.dk/blog/?p=1922)  

Instead of having to do some `location.hash` [trickery to change around the page with JavaScript](http://seanmonstar.com/2022/07/28/2010-05-13-making-an-ajax-single-page-site-with-mootools.html), there is a new API for the History object that will support this natively.

Basically, you’ll be changing the URL itself, without causing page refreshes.

