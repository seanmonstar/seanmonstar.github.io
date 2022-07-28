---
layout: post
title: What is Shipyard?
date: '2011-09-19T10:33:06-07:00'
tags:
- javascript
- programming
- shipyard
- mvc
- mozilla
- planet
tumblr_url: https://seanmonstar.com/post/10407228296/what-is-shipyard
---
[What is Shipyard?](https://github.com/seanmonstar/Shipyard)  

This week, at our Mozilla All Hands, I shared [some slides about Shipyard](https://docs.google.com/present/edit?id=0AbkIbW9hDUKXZGRwMnpuaHNfNmNyenNyNmhx&hl=en_US), a JavaScript MVC framework that is making it’s way into [Add-on Builder](https://builder.addons.mozilla.org). It’s not finished, but since I shared it there, it felt appropriate to share what there currently is here.

### Summary

An application framework that covers all the common things any JavaScript application would need to deal with: interacting with a server, storing data, rendering said data in the browser, and responding to user actions. An application built on Shipyard should only have to write the parts to pull all those things together.

If you’re application is going to have 1000 lines of JavaScript, would you rather write all those yourself, or have 900 of them be in a framework that is tested and used by others?

When starting a web application, you would reach for Django, or CakePHP, or Rails; never would you decide to use just the language itself. Why shouldn’t you do the same when the target language is JavaScript?

### Framework-wide Goals

1. Be able to declare dependencies inside the code, and not be bothered with managing them during development or deployment.
2. Be easily testable, using a node test runner.
3. Not reliant on any other language. Build scripts will use JavaScript. The End.

### More

It’s heavily influenced by MooTools, since they have an excellent modular design, but turned into modules while Moo 2.0 figures itself out. There’s [slides](https://docs.google.com/present/edit?id=0AbkIbW9hDUKXZGRwMnpuaHNfNmNyenNyNmhx&hl=en_US), and the [repository](https://github.com/seanmonstar/Shipyard), again.

