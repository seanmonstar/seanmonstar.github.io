---
layout: post
title: Firefox for Windows on ARM
date: '2012-05-11T14:52:01-07:00'
tags:
- windows
- firefox
- mozilla
- ios
- planet
tumblr_url: https://seanmonstar.com/post/22860664771/firefox-for-windows-on-arm
---
[Firefox for Windows on ARM](http://weblogs.mozillazine.org/asa/archives/2012/05/firefox-on-windows-o.html)  

Asa Dotzler:

> On ARM chips, Microsoft gives IE access special APIs absolutely necessary for building a modern browser that it won’t give to other browsers so there’s no way another browser can possibly compete with IE in terms of features or performance.

This is huge. It’s not that it’s difficult to make a Metro-style app. It’s that some core underlying APIs that users take for granted aren’t available. An easy one to notice: Firefox on ARM won’t be able to dynamically load and execute generated code; commonly known as JIT compilation. JIT compilation of JavaScript is how all modern browsers are able to make JavaScript run so quickly. Users will certainly notice that Firefox seems slow, and think it’s all Mozilla’s fault.

Then, [John Gruber points this out](http://daringfireball.net/linked/2012/05/10/dotzler-firefox-windows-arm):

> In other words, Microsoft is setting policies for Windows for ARM that are a lot like Apple’s policies for iOS. These policies and restrictions make just as much sense for Microsoft as they do for Apple.

I can never understand people who have the mind of “that’s fine, it’s better for the parent company.” It’s also _worse_ for the user. The policy that Apple has for the iOS store is just as bonkers.

