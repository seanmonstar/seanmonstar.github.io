---
layout: post
title: Object.observe
date: '2012-08-23T18:28:48-07:00'
tags:
- javascript
- programming
- mvc
- shipyard
tumblr_url: https://seanmonstar.com/post/30072647184/objectobserve
---
[Object.observe](http://wiki.ecmascript.org/doku.php?id=strawman:observe)  

This [proposed API](http://wiki.ecmascript.org/doku.php?id=strawman:observe) sounds fantastic. Property observing is a key component of data-binding, used in places like [Ember](http://emberjs.com), [Shipyard](http://seanmonstar.github.com/Shipyard), and [Angular](http://www.angularjs.org/). Unfortunately, it requires using special `get` and `set` methods, and only on objects derived from the library; using a regular `{}` or `[]` doesn’t work.

Object.observe would make it simply part of JavaScript, likely be more performant than any of the libraries, and allow developers to use plain native objects. The proposed usage is this:

    Object.observe(foo, function(changes) {
        changes.forEach(function(c) {
            // c === { name: 'bar', type: 'new', object: foo }
        });
    });

There’s a [video](https://www.youtube.com/watch?v=VO--VXFJnmE) and [repo](https://github.com/rafaelw/v8) for a working prototype of this stuff too.

