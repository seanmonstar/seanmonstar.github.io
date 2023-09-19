---
layout: post
title: Cross-browser addEvent without Frameworks
date: '2008-10-01T18:49:00-04:00'
tags:
- javascript
tumblr_url: https://seanmonstar.com/post/706917892/cross-browser-addevent-without-frameworks
---
![addEvent](http://monstar.blazonco.com/images/blog/addevent.jpg)

Javascript is a very event-driven language. Considering the interactivity of the medium (the web), the most common need of Javascript by far is to respond to user interaction. Unfortunately, a problem that plagues Javascript that most other languages take for granted, is that the standard library isn’t very standard. So even the way to add event listeners in Javascript, a function of utmost importance, is implemented differently among browsers. So how do we fix this without a framework?

If you haven’t done much Javascript programming without a framework, it’s easy to miss what all a framework is doing for you. And sometimes, there isn’t enough Javascript functionality needed to warrant the overhead kilobytes of a framework. So, what really goes on underneath? Well, I’m not going to show you everything, because a lot of _convenience can be put into wrapper functions_ .

To simply make adding event listeners easier in all browsers, we make one function, I’ll call it addEvent ’cause I’m used to Mootools.

    window.addEvent = function(event, target, method) { if (target.addEventListener) { target.addEventListener(event, method, false); } else if (target.attachEvent) { target.attachEvent("on" + event, method); } }

It takes for parameters a _string of the event_ , the _target element_ , and the _function to execute_ . We’re checking which function exists in the browser. addEventListener is most standard browsers, and attachEvent is for IE. It also takes care of adding “on” to the start of the Event for IE.

Now then, you can access this function in another function by calling window.addEvent().

    var someFunc = function() { var el = document.getElementById('someEl'); window.addEvent('click',el,funtion(){ //do stuffz... }); var formEl = document.getElementById('someFormEl'); window.addEvent('submit',formEl,funtion(){ //do stuffz... });}

This is the simplest way, as we just create a global function. Frameworks will add to the convenience by allowing you to call Element.addEvent instead of passing the element as a parameter. But this works fine when you just have a few simple Events to watch in a simple script.

Any tips or convenience that you add to your own addEvent function?

