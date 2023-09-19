---
layout: post
title: IE Opacity Ignores Positioned Children
date: '2010-04-12T09:41:00-04:00'
tags:
- bug
- javascript
- css
tumblr_url: https://seanmonstar.com/post/709013028/ie-opacity-ignores-positioned-children
---
The CSS2 Opacity property doesn’t work in the current versions of Internet Explorer - through IE8. However, it has provided a way to achieve similar results with a different method, using IE’s filter property. Javascript frameworks usually work this in for you when you try to set an elements opacity in a cross-browser fashion.

It should come as no surprise that there are bugs that arise from this filter property. One is that **the filter doesn’t cascade to children that have their position as anything besides static**.

This has been talked about in several places all over the internets, and even recently in the MooTools User Group, but it seemed worthy of documenting if only for my own purposes. You can see for yourself with this [demo at jsFiddle](http://jsfiddle.net/rpflorence/ZDnCu/).

A way to fix this has been suggested multiple times as well: You can set the positioned element to inherit the filter. Unfortunately, the instance where I ran into this bug proved unsolvable, because I actually needed that filter property to do something besides inherit. Nonetheless, if you’re simply trying to fade out and find that part of it isn’t fading in IE, this will solve it. This, and apparently some sort of change to the layout of the element is also required. You could change its display to inline-block, but some felt that setting contentEditable to true to be the least intrusive on changing the layout. I’m not so sure I agree, but [here is it anyways](http://jsfiddle.net/L6n3H/).

    #relative {    
    	position: relative;    
    	filter: inherit;
    }

