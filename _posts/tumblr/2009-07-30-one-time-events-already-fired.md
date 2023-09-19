---
layout: post
title: 'One Time Events: Already Fired?'
date: '2009-07-30T13:18:00-04:00'
tags: []
tumblr_url: https://seanmonstar.com/post/707073408/one-time-events-already-fired
---
[Batch programming](http://en.wikipedia.org/wiki/Batch_programming) is certainly very easy to understand and write. You order a todo list and give it to the compiler/interpreter. It’s what’s usually taught first when you learn to program, and even in some server-side languages, you batch program (PHP doesn’t have events at all, for instance). However, persitance applications that live on the desktop (and perhaps Ajax applications with minimal page refreshes) rely on [event-driven programming](http://en.wikipedia.org/wiki/Event-driven_programming). You set up a user interface, and then wait for the user to do something to that interface, and then respond to the event.

This is all well and great. But I’ve hit a problem a couple times recently as I create more desktop applications. I thought this would be easy to find, since I’ve hit the problem often and early. But I haven’t, so I’ll have to solve it myself. Sometimes, a component might have a one-time event that you need to listen to, and you don’t know if that event has fired or not.

As an example, I have a Database object that builds and connects, and while the Object reference is available immediately, if I send any requests to it before it’s connected, it errors. So naturally, I can listen for the `connect` event. Unfortunately, This has 2 problems, since the `connect` event will only fire once, shortly after instantiation.

- What if the event fires before I attach the listener?
- I want to run a method multiple times in the application, so an event that fires once limits the execution of those event listeners.

If the Database was particularly quick in connecting, it’s possible the `connect` event will have fired before we get to the code that sets up the connect event listener. Since `connect` will never fire again, our function will never fire.

<figure class="tmblr-full" data-orig-height="200" data-orig-width="500"><img src="https://64.media.tumblr.com/f6bfb541d3b2ca2e1ec1fefe8a88343a/d8e8a51f7cd1f451-f1/s540x810/229382dc2a4bfa4b919e8b885e71fa16229c3d9d.png" data-orig-height="200" data-orig-width="500"></figure>

This is similar to Javascript, where if you listen for the window.onload event after the page has loaded, you’re never going to execute. At least in MooTools, they solve this by having the domready event check every time you attach a listener, determine if the event has already fired, and if so, execute the listener immediately.

It’s that exact kind of behavior I expected to be easily possible. Here’s what I wanted: After instantiating my Database, later in the program when I access that reference, and try to make a query, or access something else otherwise forbidden, I’d like for it to automatically check for me if it’s accessible, or loaded, or connected, or whatever. If it has, execute my commands immediately. If it hasn’t, then assign it as an event listener. This would allow me to call this code again later, and not provide a different part for when I expect the database to be fully setup already.

Am I asking too much? I don’t think so. _My goal in programming is always to take anything I do repetitively, and make it happen automatically_. I don’t think I should have to write a check to see if something has loaded, and if not, load it and attach a listener, otherwise do now. It’s the same behavior in every instance. Perhaps some sort of addition attachListener code should be wrapped up. Or! or, using getters could also be a cool solution. I’ll have to do my research, and share my solution later as I find it.

**Have you solved this before?**

