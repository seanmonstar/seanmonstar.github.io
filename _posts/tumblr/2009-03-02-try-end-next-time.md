---
layout: post
title: 'Try end() next() time '
date: '2009-03-02T18:02:00-05:00'
tags:
- php
tumblr_url: https://seanmonstar.com/post/707134387/try-end-next-time
---
Here’s a short tip today. I’ve been finding that when using `foreach` in PHP to check if there’s more in the array, the use of `next()` has _failed me on multiple occasions_ . I just converted every instance of `next()` in my code to instead use [`end()`](http://php.net/end).

    if($cat !== end($categories)) { echo ", "}

This way, I don’t need to worry about the internal pointer being pushed around before the loop, or during the loop. I really only care if the value I’m abusing is the last value, anyways.

