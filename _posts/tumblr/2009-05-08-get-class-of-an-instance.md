---
layout: post
title: Get Class of a MooTools Instance
date: '2009-05-08T11:00:00-07:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/707118236/get-class-of-an-instance
---
Have you ever wanted to get the class name of an object, maybe to check what instance it is of, or maybe to use the name for something? I was in need of exactly that, so I set out to do so last night. I started by asking [Stack Overflow](http://stackoverflow.com/questions/837729/how-to-get-the-name-of-a-mootools-class-from-within), and no one gave me an answer, so I pondered about it a bit.

_Updated: 6/16/10_

I eventually solved it, like the idiot I am. I solved it **wrong**.

My solution caused an infinite loop in some browsers, and errors in others as it tried to access properties of the window object that are “illegal”.

A [version currently exists](http://github.com/seanmonstar/MonstarLab-MVC/blob/master/src/mvc/GetClass.js) that can’t find anything that isn’t a direct property of the window global object. I’d eventually like it to be able to find out if the class is called Drag.Move, for instance.

