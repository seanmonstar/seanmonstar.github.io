---
layout: post
title: syslogger
date: '2014-01-02T19:00:30-05:00'
tags:
- javascript
- nodejs
- programming
- logging
- syslog
- planet
tumblr_url: https://seanmonstar.com/post/72018297115/syslogger
---
When recently writing an [intel-syslog](https://npmjs.org/package/intel-syslog) library, I noticed that somehow, npm was lacking a sane syslog library. The popular one, [node-syslog](https://npmjs.org/package/node-syslog), is a giant singleton, meaning it’s impossible to have more than one instance available. That felt wrong. Plus, it’s a native module, and for something so simple, I’d rather not have to compile anything.

That’s where [syslogger](https://npmjs.org/package/syslogger) comes in. It’s pure JavaScript, and has a simple API to allow you to create as many instances as you’d like.

    var SysLogger = require('syslogger');
    var logger = new SysLogger({
      name: 'myApp',
      facility: 'user',
      address: '127.0.01',
      port: 514
    });
    
    logger.log(severity, message);
    // or
    logger.notice(message); //etc

[Enjoy](https://npmjs.org/package/syslogger).

