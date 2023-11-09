---
hidden: true
layout: post
title: A new direction for web applications
date: '2012-01-24T15:10:18-05:00'
tags:
- javascript
- node
- programming
- planet
tumblr_url: https://seanmonstar.com/post/16419650738/a-new-direction-for-web-applications
---
[A new direction for web applications](http://www.mikealrogers.com/posts/a-new-direction-for-web-applications-.html)  

As [I said last night](https://twitter.com/seanmonstar/status/161678251796865024), I don’t see how nodejs is _the_ solution, by itself, for a web application that talks to many services. I’d still say that MVC is the best solution we have seen so far. Python has great support for long-running async tasks, talking to various databases, and the like.

That said, I love my JavaScript. Here’s where I see nodejs being a win: writing JavaScript on both sides of the stack feels great, and it gains small amounts of CPU time by making file I/O asynchronous. For other long-lasting tasks, such as resizing an image, you still need to set up a task in a queue for other CPUs to crunch.

