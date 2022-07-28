---
layout: post
title: How ECMAScript 5 still does not allow to subclass an array
date: '2010-07-16T06:59:45-07:00'
tags:
- javascript
tumblr_url: https://seanmonstar.com/post/819507208/subclass-an-array
---
[How ECMAScript 5 still does not allow to subclass an array](http://perfectionkills.com/how-ecmascript-5-still-does-not-allow-to-subclass-an-array/)  

Extending an Array in JavaScript appears to be quite the hassle. I had always wondered why the jQuery object didn’t set Array as its prototype.

MooTools solves the issue with their `Elements` class, which basically copies all of `Array`’s methods onto it, so it behaves just like one.

