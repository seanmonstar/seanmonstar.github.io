---
layout: post
title: 'insist: Better assertions for nodejs'
date: '2013-12-11T13:01:39-05:00'
tags:
- javascript
- nodejs
- planet
- programming
- insist
- assert
tumblr_url: https://seanmonstar.com/post/69703845045/insist-better-assertions-for-nodejs
---
[insist: Better assertions for nodejs](https://npmjs.org/package/insist)  

There’s plenty of [assertion libraries](https://npmjs.org/browse/keyword/assert) out there that give you all brand-new APIs with all sorts of assertion testing. I don’t need that. I actually like just using the `assert` module. Except one thing. The standard message for `assert(false)` is useless.

I saw `better-assert`, got excited, and then realized it only provided one method. All I wanted was the `assert` module, with a better message.

So I made [`insist`](https://npmjs.org/package/insist). You can truly just drop it instead of `assert`, and pretend it’s `assert`. Also, unlike a few libraries who try to do this, `insist` plays well with multi-line assertions.

    var assert = require('insist');
    var someArr = [15, 20, 5, 30];
    assert(someArr.every(function(val) {
      return val > 10;
    }));
    // output: AssertionError: assert(someArr.every(function(val) {
    // return val > 10;
    // }));

I [insist](https://npmjs.org/package/insist).

