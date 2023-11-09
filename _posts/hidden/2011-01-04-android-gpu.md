---
hidden: true
layout: post
title: The Care and Feeding of the Android GPU
date: '2011-01-04T12:17:13-05:00'
tags:
- android
- ux
- gpu
tumblr_url: https://seanmonstar.com/post/2597051626/android-gpu
---
[The Care and Feeding of the Android GPU](http://www.satine.org/archives/2011/01/01/the-care-and-feeding-of-the-android-gpu/)  

Charles Ying:

> Android’s UX architecture needs work. UI compositing and the view system are both primarily done in software. Garbage collection and async operations frequently block UI rendering.

As much as I like Android, this is definitely a low point. It gets rather frustrating when an app doesn’t respond because of some blocking URL request, or a huge amount of garbage collection is happening. I _always_ want the phone to respond to my touch, at the very least.

