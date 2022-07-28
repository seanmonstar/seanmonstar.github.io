---
layout: post
title: Boolean Arguments
date: '2011-04-21T11:00:05-07:00'
tags:
- programming
- java
- python
- booleans
- code style
tumblr_url: https://seanmonstar.com/post/4810862963/boolean-arguments
---
I find that boolean arguments don’t read that well when revisiting the code a little later.

    doBackup(true);

This is easily fixed with languages that allow [keyword arguments](http://seanmonstar.com/post/3746460491/function-kwargs), like Python:

    doBackup(notifyOnSuccess=True)

However, in languages without, such as Java, I’ve found it preferable to create two constants for true and false, named for what they will be used for.

    private static final boolean DONT_NOTIFY = false;
    private static final boolean DO_NOTIFY = true;
    
    doBackup(DO_NOTIFY);

It’s kind of like the suggestion in [Code Complete](http://www.amazon.com/gp/product/0735619670?tag=seanmonstar-20) to use Enums instead of booleans.

