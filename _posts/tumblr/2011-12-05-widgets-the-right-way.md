---
layout: post
title: Widgets the Right Way
date: '2011-12-05T21:21:03-05:00'
tags:
- widgets
- javascript
- html
- social buttons
tumblr_url: https://seanmonstar.com/post/13807757815/widgets-the-right-way
---
[Widgets the Right Way](http://24ways.org/2011/defending-the-perimeter-against-web-widgets)  

A lot of sites are using widgets more, be it for ads, or these terrible social buttons that have emerged. Besides not wanting to spend time downloading the social buttons that I won’t use, what I hate the most is that they all have this in common: If they lag, they stall the rest of the page.

That’s because they `document.write` their contents into the page, which prevents all further rendering until they’ve loaded. Don’t do that. Read this from Rich Thornett about how to make your widgets suck less.

