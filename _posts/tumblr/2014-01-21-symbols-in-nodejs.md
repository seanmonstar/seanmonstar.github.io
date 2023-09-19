---
layout: post
title: Symbols in nodejs
date: '2014-01-21T22:04:15-05:00'
tags:
- javascript
- programming
- nodejs
- symbol
- es6
- planet
tumblr_url: https://seanmonstar.com/post/74130530265/symbols-in-nodejs
---
[Symbols in nodejs](https://npmjs.org/package/symbol)  

Symbols are a new feature in ES6 that allows us to set true private properties on object, without fear of name conflicts. The cool thing is, you can kind of do it already in ES5, and the [symbol](https://npmjs.org/package/symbol) module offers it for nodejs.

    var Symbol = require('symbol');
    var key = new Symbol;
    var obj = {};
    obj[key] = "foo"; // no one can access without a reference to `key`

