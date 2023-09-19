---
layout: post
title: Make the DOM Update Faster
date: '2010-02-12T11:45:00-05:00'
tags:
- javascript
- dom
- bestof
tumblr_url: https://seanmonstar.com/post/708943208/make-the-dom-update-faster
---
We’ve already learned that to make our Javascript load faster, we should be listening for a [domready event](http://mootools.net/docs/core/Utilities/DomReady) instead othe window onload event. However, sometimes, you can actually make too much Javascript reliant on that sacred event. And when you do that, you can get quite the jump or flicker in older browsers.

We use a proprietary text replacement program instead of sIFR or Cufon or anything else out there. We call it Typostream. On one of my recent projects, we had several features of the web-site requiring extra Javascript functionality, along with a good portion of text being replaced. Originally, I had all of this Javascript being executed on DOM ready event, as best practices recommend. However, viewing the site in Internet Explorer revealed some amazingly laggy results.

#### Doing a lot at domready can suck

In IE (even IE8), the time it took to execute all the Javascript was way too long, in the realm of a several seconds too long. Now, the ultimate cuplrit was that we had over 100 elements on the page that needed to be swapped out for images of the same text. Eventually, we wear able to alter the design to require only a couple parts of the page to be replaced. But I discovered a more immediate fix for such a big flicker of content.

    LOG: Total: 244, Since Last: 244 LOG: Total: 1992, Since Last: 1748

<small>The timer was started at the top of of the Javascript file. The first profile call was at the start of the DOM ready event, and the second call was after the Typostream call.</small>

Basically, **with some logging, I found out where the big beasty functions were**. On the inside, the common mechanism for dom ready implementations, is that every function you add as a “listener” to dom ready, just gets pushed into an array. Once dom ready fires, [it loops through the array and executes all the functions](http://github.com/mootools/mootools-core/blob/master/Source/Element/Element.Event.js#L96). So basically, the dom ready event firing can become one giant function that needs to finish.

    var $log = function() {
    	if(window.console && console.log) {
    		for(var i = 0; i < arguments.length; i++) {
    			console.log(arguments[i]);
    		}
    	}
    };
    var $profile = (function() {
    	var _start = new Date(),
    	_last = new Date();
    	return function(msg) {
    		var now = new Date();
    		msg = (msg || '') +' ,';
    		$log(msg + 'Total: '+ (now - _start) +', Since Last: ' + (now - _last));
    		_last = now;
    	};
    })();

<small>This was my method of profiling. You could also use <code>console.time</code> and <code>console.timeEnd</code>, but this works in a pinch (and in older browsers).</small>

Now, many things we do in Javascript are setting up event listeners and timers, so the major work at dom ready is usually fairly small. But the amount of work that was needed to swap out all those elements (and having to do slow selector lookups in the older versions of IE) was all trying to execute at dom ready. And while the DOM was being accessed and changed during that time period, the Javascript hadn’t given up its execution to allow the browser to update the DOM.

**The DOM was only able to update itself after that Javascript had executed**. So even the smaller part at the start, like collapsing an accordion, wasn’t happening until several seconds later. The solution was to remove how much Javascript was happening on the DOM ready event. I wanted to break it up, giving the DOM the ability to do all the changes for the accordion and modal boxes, _before trying to do the heavy lifting of running up and down the DOM tree a hundred times_.

#### Well, then move out of current execution

Along comes `setTimeout`. By wrapping the typographical replacement into a timeout call, I gave the browser an opportunity to run other browser stuff. It does its thing (like fixing most of the page immediately), and then calls my big function afterwards. Sure, that big function will still take a couple seconds to execute, making the text flicker, but at least its not as glaring as the section navigation collapsing 5 seconds in. And like I said above, we were able to drastically reduce the number of elements to change, thus fixing the problem even more.

But it’s useful to know that you can actually clog up your `domready` event, and how to clean it out again. If you had a **ton of things that needed to happen** , you could probably set up a simple queue system that will execute everything you want in order, but setting a timeout for each one. Or you could try out web workers, if that fits your bill.

