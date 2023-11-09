---
hidden: true
layout: post
title: Callbacks vs Events
date: '2010-07-05T13:00:00-04:00'
tags:
- javascript
tumblr_url: https://seanmonstar.com/post/773331630/callbacks-vs-events
---
[Callbacks vs Events](http://dean.edwards.name/weblog/2009/03/callbacks-vs-events/)  

Dean Edwards points out that using an array and looping through callbacks is _brittle_:

    $(document).ready(function() {
      console.log("Init: 1");
      DOES_NOT_EXIST++; // this will throw an error
    });
    
    $(document).ready(function() {
      console.log("Init: 2");
    });</pre>

> What do we see in the console?

    Init: 1
    
    Error: DOES_NOT_EXIST is not defined

> The problem is clear. Callback systems are brittle. If any of the callback functions throws an error then the subsequent callbacks are not executed. In reality, this means that a poorly written plugin can prevent other plugins from initialising.

