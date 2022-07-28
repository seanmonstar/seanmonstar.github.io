---
layout: post
title: JavaScript Module Syntaxes
date: '2011-10-19T16:03:00-07:00'
tags:
- javascript
- programming
- shipyard
- mozilla
- planet
tumblr_url: https://seanmonstar.com/post/11670322838/javascript-module-syntaxes
---
[JavaScript Module Syntaxes](http://blog.calyptus.eu/seb/2011/10/choosing-a-javascript-module-syntax/)  

Excellent analysis of current module patterns by fellow MooTooler [Sebastian Markbåge](http://blog.calyptus.eu/seb/2011/10/choosing-a-javascript-module-syntax/).

His conclusions match mine, and I used them in [Shipyard](https://github.com/seanmonstar/Shipyard) profusely.

1. In a production environment, you will always want to minify and concatenate your scripts. Since there is always a “build” step, all syntaxes are equal in production.

2. In development, what matters most is a syntax that gets out of the way of programming, and more importantly, testing your code. That means that AMD is out. CommonJS modules have less boilerplate, and work immediately in other environments, like nodejs. This means you can very easily drop into a terminal, run nodejs, and import your module without a hitch.

