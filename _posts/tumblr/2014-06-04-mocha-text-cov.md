---
layout: post
title: mocha-text-cov
date: '2014-06-04T12:10:01-07:00'
tags:
- javascript
- programming
- nodejs
- mocha
- planet
tumblr_url: https://seanmonstar.com/post/87817275237/mocha-text-cov
---
[mocha-text-cov](https://www.npmjs.org/package/mocha-text-cov)  

I’ve been looking for a super simple reporter for [Mocha](http://visionmedia.github.io/mocha/) that shows a summary of code coverage in the console, instead of requiring me to pipe to a file and open a browser. I couldn’t find one. So [I made mocha-text-cov](https://www.npmjs.org/package/mocha-text-cov).

Here’s a truncated example, if you don’t want to click through:

    > mocha --ui exports --require blanket --reporter mocha-text-cov
    
      Coverage Summary:
      Name Stmts Miss Cover Missing
      ---------------------------------------------------------
      lib\config.js 89 0 100%
      lib\console.js 140 0 100%
      lib\filter.js 23 0 100%
      lib\filterer.js 22 0 100%
      lib\formatter.js 57 4 93% 37,54,55,97
      lib\index.js 44 0 100%
      lib\levels.js 16 0 100%
      lib\logger.js 155 0 100%
      lib\record.js 67 5 93% 97,98,99,100,101
      lib\utils\json.js 16 5 69% 10,11,12,13,14
      =========================================================
      TOTAL 629 14 98%

