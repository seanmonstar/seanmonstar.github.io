---
layout: post
title: console.log() all the things!
date: '2013-07-25T15:55:00-04:00'
tags:
- nodejs
- programming
- logging
- javascript
- planet
- bestof
tumblr_url: https://seanmonstar.com/post/56448644049/consolelog-all-the-things
---
Let’s talk about logging. We like logging things. Logging to console, to files, to syslog, maybe even sending some over email. Formatting our logging is pretty fun. Oh, the colors! Of course, we must colorize our logs. Or not, if your colorist. But there’s a problem with logging. There is? Yup. We can’t agree how everything should log its logs.

Let’s look at this from two levels: **libraries** and **apps**.

If you follow The Node Way, then you make and use focused packages or libraries from npm to accomplish specific tasks. Sometimes, those tasks need to tell the developer what happened, or otherwise record for eternity their minor quibbles. So we end up with libraries that want to log things.

The modules are no good without plugging them together to make an app. Many apps are web apps, but they could be anything, like flying nodecopters or some such. So you have a bunch of app specific code tying together libraries, and you want to log all over the place. You want to know response codes, response times, when errors occur, when people attack your app, and when unicorns invade.

### So what’s the problem?

At the end of the day, you want to ship an app. We all ship apps. And while you were devving away on your machine, logging to console was perfectly fine. But once you start shipping, you can’t just watch the console blast by on hundreds of production machines like The Matrix. This is typically when you decide that you need a configurable logging library, so you can rotate log files, send some to syslog, email exception logs, and fill them full of colors.

To use a real example, we’ll play with [winston](https://npmjs.org/package/winston). To use winston, we create a logger, specify some transports, and then pass the logger all over the application.

    const winston = require('winston');
    var logger = new (winston.Logger)({ transports: [
        new (winston.transports.File)({ 
            filename: '/var/log/app.log', 
            colorize: true 
        }),
        new (winston.transports.Console)({ colorize: true })
    ] });
    module.exports = logger;

Then, elsewhere, you would require that module we just defined, and log stuff.

    const logger = require('./lib/logger');
    logger.info('blast off');

This works for a while in your own app, but you’ll notice that you can’t make any of your dependencies use your logger. It’s also pretty terrible when you decide you want to uplift one of your `lib` modules into a standalone package. You realize you still want the logging messages, but now you can’t depend on the app giving you a logger. There goes another unicorn.

We’re never going to agree on a library that all apps and libs should use, and we shouldn’t! Competition, blah blah, etc. I’ll walk through probably the most obvious solution, show why it’s not good enough, and then propose the real solution.

### Log, log, pass?

Libraries could accept a logger option. This works fine when the library provides a constructor. It’s terrible when the library simply provides a handful of static utility functions. Those that do have constructors could allow a `logger` option, but default to `console`, and still benefit from the real solution below.

I propose, instead, that all libraries just use `console.log` and logging libraries overload `console.log`. Well, clearly, overload all of `console`. Craziness? Maybe, but then we already get paid to write JavaScript.

### console.log()

In node.js, the `console` object comes with `debug`, `log`, `info`, `warn`, and `error` methods already. So a library can depend on this universal global, and log things at the correct levels. The application, at the main module, can create a fancy pants logger, and overload `console` with it. Now, all library code is pumping its logs through your fancy pants, and is none the wiser. How fancy.

Here’s how we’d do that with our winston logger:

    const winston = require('winston');
    
    var logger = new (winston.Logger)({
      // transports and whatever loggy stuffs
    });
    
    logger.extend(console);
    
    var log = console.log;
    console.log = function hijacked_log(level) {
      if (arguments.length > 1 && level in this) {
        log.apply(this, arguments);
      } else {
        var args = Array.prototype.slice.call(arguments);
        args.unshift('debug');
        log.apply(this, args);
      }
    }

So say we all?

