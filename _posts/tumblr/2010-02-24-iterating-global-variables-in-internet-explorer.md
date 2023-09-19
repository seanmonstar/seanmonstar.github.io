---
layout: post
title: Iterating Global Variables in Internet Explorer
date: '2010-02-24T11:02:00-05:00'
tags:
- javascript
- bug
tumblr_url: https://seanmonstar.com/post/708979238/iterating-global-variables-in-internet-explorer
---
There’s a couple ways you can define globally accessible variables in Javascript. And it turns out that in JScript, they actually mean different things (as opposed to all other implementations, where they’re the same). This meant my [GetClass implementation](http://seanmonstar.com/blog/2009-05-08-get-class-of-an-instance/) just plain wouldn’t work for Internet Explorer. Well, that’s no good, since that’s a basic building block of my [MooTools MVC framework](http://github.com/seanmonstar/MonstarLab-MVC). Now, I could require all classes be created explicitly, like `window.Task`, but that makes for a very inflexible pattern. And there’s no reasonable way to explain to users why I’m requiring that.

So instead, I delved into JScript to find a way to let me iterate all global variables. But first, here’s a few ways you could declare a global variable.

    var a = 1;
    window.b = 2;
    c = 3;
    (function(global) {    
    	this.d = 4;    
    	global.e = 5;
    })(window)

**Only if you specifically set the property of the window object does it get attached**. The cases that are declaring variables instead of properties get added to a Variables Host object instead. Frankly, this is awful. But we come to expect this kind of shenanigans from IE.

This causes this iteration bug:

    for(var i in window) {
    	i; 
    	//besides the usual, we'll only get
    	//b, d, and e.
      }

Thankfully, some other people have noticed this before me. [Eric Lippert](http://blogs.msdn.com/ericlippert/archive/2005/05/04/414684.aspx) did much more extensive research than I needed to fix the issue. But his research assisted Thomas Frank in a way that solved my problem as well.

> This should get all our global variables, but doesn’t in JScript/IE.
> 
> var x=[];
> for (var i in windows){
> x.push(i)
> };
> alert(x.join("\"));
> 
> Well, it turns out this doesn’t work in JScript/IE at all, because non-native variables are not found when we try to enumerate the window object.
> 
> [—Find your global namespace](http://www.thomasfrank.se/global_namespace.html)

This might not necessarily be the best solution, but using a cache, I only have to eat the execution cost once. Thomas’ solution was to search all script tags for all variable declarations. This works because even though `a` in my first example doesn’t get iterated through, it can still be found through a property lookup on the window global object (`window['a']`).

> The first thing I had to do was to write a workaround for IE browsers. In IE it is possible to get the complete script source for a page by looking at the `document.scripts` array.
> 
> If an element has no `innerHTML` value, then it is an external script and we can load the source with a simple AJAX request. We then use a little bit a regular expression magic to extract every “word” from the source(s) and check if these words exists as global variables. This works suprisingly well.

Using his approach, modified to store all the “word” possibilities in a cache so I don’t have do more ajax requests, I’ve cooked up the following bit to implement a sort of `indexOf` (or `keyOf`) for the window object.

    var keyOf = (Browser.Engine.trident) ? (function() {
    	var xhr = function(path) {
    		var request = window.ActiveXObject ? new ActiveXObject("Microsoft.XMLHTTP") : new XMLHttpRequest();
    		request.open('GET', path, false); //asynchronous = false
    		try {
    		request.send();
    		} catch(e) { return null; }
    		if (request.status == 500 || 
    		request.status == 404 || 
    		request.status == 2 ||
    		(request.status == 0 && request.responseText == '')) 
    		return null;
    		return request.responseText;
    	}
    	var cash = {};
    	var ie_search = function(obj) {
    		var key = search(obj) || search(obj, cash); //search does a for..in loop on window.
    		if(!key) {
    			//IE doesn't enumerate `var a = 'x';`, so we need to explicitly
    			//declare those vars on the window object
    			// credit: http://www.thomasfrank.se/downloadableJS/globalVars.js
    			var scripts = document.scripts,
    			src = '';
    			for(var s = 0, len = scripts.length; s < len; s++) {
    				if(!scripts[s]._lookedUp) {
    					if(scripts[s].innerHTML) {
    						src += scripts[s].innerHTML;
    					} else if (scripts[s].src) {
    						src += xhr(scripts[s].src);
    					}
    					scripts[s]._lookedUp = true;
    				}
    			}
    			var idents = src.replace(/\\W/g, ' ').replace(/(function|if|for|while|true|false|null|typeof|var|new|try|catch|return|prototype|this)/g, '').split(' ');
    			for(var i = 0; i < idents.length; i++) {
    				var iden = idents[i].trim();
    				cash[iden] = true;
    				if(!key && (iden in window) && window[iden] == obj) {
    					key = idents[i];
    				}
    			}
    		}
    		return key;
    	}
    	return ie_search;
    })() : search;

The first time you call `keyOf`, it will collect all the Javascript source for the page, and then store every possible identifier that was declared. On subsequent calls, only new scripts will be gotten and parsed. Since the values of the global variables could change at any time, I could only store in the cache their names. The search function tries all those names on the window object.

A definite downside I see to this method is since it uses XHR to collect the source, it wouldn’t work on scripts that aren’t from the same domain. But for [my framework](http://github.com/seanmonstar/MonstarLab-MVC/blob/master/src/mvc/GetClass.js), since it uses a loader anyways, it’s sufficient for me.

