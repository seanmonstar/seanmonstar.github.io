---
layout: post
title: Writing Testable JavaScript
date: '2010-07-12T12:57:46-07:00'
tags:
- javascript
tumblr_url: https://seanmonstar.com/post/803200989/writing-testable-javascript
---
[Writing Testable JavaScript](http://www.adequatelygood.com/2010/7/Writing-Testable-JavaScript)  

Some good advice about singletons, since their popular with the Module Pattern. Also true about “tight” functions, but this is true in any language.

However, I’d ignore the tip to not use private functions just so their testable. That argument doesn’t fly in any other language, and the only thing the rest of your program cares about is that the public interface does what it says it does. That’s the point of unit testing: **guaranteeing the public API contract.**

