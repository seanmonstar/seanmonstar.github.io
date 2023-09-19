---
layout: post
title: Fade and Destroy Elements
date: '2009-09-30T18:30:00-04:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/707205823/fade-and-destroy-elements
---
One of my favorite methods that I use constantly in various interfaces combines fading and removing of an element. It could be a table row, or list item, or a div, but often times when I wanted to remove something, _I’d like it to remove with a fade_. And not always wanting to write to listen for the complete event and then remove the element.

    Element.implement({
    fadeAndDestroy: function(duration) {
    	duration = duration || 600;
    	var el = this;
    	this.set('tween', {
    		duration: duration
    	}).fade('out').get('tween').chain(function() {
    		el.dispose();
    	});
    }});

You’d proceed to use this combo function like so:

    $('elem').fadeAndDestroy();

