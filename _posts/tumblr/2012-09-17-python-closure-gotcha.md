---
layout: post
title: Python Closure Gotcha
date: '2012-09-17T16:57:44-04:00'
tags:
- programming
- python
- planet
tumblr_url: https://seanmonstar.com/post/31749159302/python-closure-gotcha
---
You know all about [JavaScript closures](https://developer.mozilla.org/en-US/docs/JavaScript/Guide/Closures). They’re great. We can do [super cool stuff](http://seanmonstar.com/blog/closures-break-my-for-s/) with them. Python has closures, too! However, the combination of closures and no `var` keyword leads us to a fun Gotcha.

Example of Gotcha:

    def something_awesome():
        awesome = 1
        def increase_awesomeness():
            awesome = 2
    
        increase_awesomeness()
        return awesome

You’d think that this would return 2, but it returns 1. This is due to Python creating a new local variable `awesome` inside the inner function. You _can_ reach out of the closure when accessing a property. In places like these, since the inner function is used for utility only, I attach a property to it.

    def something_awesome():
        def increase_awesomeness():
            increase_awesomeness.awesome += 1
    
        increase_awesomeness.awesome = 1
        increase_awesomeness()
        return increase_awesomeness.awesome

