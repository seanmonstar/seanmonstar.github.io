---
hidden: true
layout: post
title: Asynchronous UIs
date: '2011-11-30T19:11:14-05:00'
tags:
- javascript
- mvc
- ui
- ajax
- programming
tumblr_url: https://seanmonstar.com/post/13564996626/asynchronous-uis
---
[Asynchronous UIs](http://alexmaccaw.co.uk/posts/async_ui)  

A great post by Alex McCaw, creator of the [Spine](https://github.com/maccman/spine) MVC JavaScript<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup> framework, about how front-end developers do Ajax interactions wrong. Users shouldn’t be stopped because we’re waiting on the server to respond. The interface should respond immediately, and a background sync action should be kicked off.

I’ve been thinking about this for the past year, and wanted to reflect these ideas in [Shipyard](https://github.com/seanmonstar/Shipyard), and make them come alive in the Add-on Builder, myself.

* * *

1. 

I refuse to use CoffeeScript, so it’s off limits to me.&nbsp;[↩︎](#fnref:1)

