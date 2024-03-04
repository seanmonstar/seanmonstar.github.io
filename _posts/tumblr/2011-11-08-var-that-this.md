---
layout: post
title: var that = this
date: '2011-11-08T15:00:05-05:00'
tags:
- javascript
- programming
- opinion
- planet
- closures
tumblr_url: https://seanmonstar.com/post/12521817761/var-that-this
---
I used to think say you should use `that` in the question “What variable should I name `this` for closures?” This was because `self` is already a variable that points to `window`. However, I’ve since [revised my opinion](https://plus.google.com/117864091849261637847/posts/eB94v35EtEy) on what is a good variable name in this case.

I now find `that` and `self` to be too vague.

Instead, I think the variable should be named after the Class that you are currently coding in. As always, it’s easier to explain with code:

    var PackageController = new Class({
    
        doSomething: function() {
            var controller = this;
    
            someEl.addEvent('click', function(e) {
                controller.react();
            });
        }
    
    });

The reason is because once you start delving a couple nested functions deep[^1], I find myself sometimes wondering if I bound `that` to the value I wanted. This way leaves little to wonder about. And since you read code far more than you write it, best to write the most readable code you can.



[^1]: I know, some of that can be solved by moving the functions to named methods on other objects, but you’d be lying if you said you never happened to have a function for a `.forEach` loop, and then another inside for an `.addEvent`, or something similar.

