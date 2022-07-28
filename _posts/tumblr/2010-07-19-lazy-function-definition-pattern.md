---
layout: post
title: Lazy Function Definition Pattern
date: '2010-07-19T07:01:43-07:00'
tags:
- javascript
tumblr_url: https://seanmonstar.com/post/832086490/lazy-function-definition-pattern
---
[Lazy Function Definition Pattern](http://peter.michaux.ca/articles/lazy-function-definition-pattern)  

Sometimes there’s setup involved with a function that we would prefer to only have to execute if the function is needed. It’s also preferable to not have to do an `if` statement every function call, since less logic is more efficient.

    var foo = function() {
        var t = new Date();
        foo = function() {
            return t; 
        }; 
        return foo();
    };

