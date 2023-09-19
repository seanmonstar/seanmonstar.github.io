---
layout: post
title: Don't Use the DOM to Insert Flash
date: '2009-04-21T10:46:00-04:00'
tags:
- javascript
- flash
- bug
tumblr_url: https://seanmonstar.com/post/707145908/dont-use-the-dom-to-insert-flash
---
I didn’t have the priviledge of using SWFObject or the likes. I just needed to create a movie in a modal-like window that depended on the page you were on. So, taking a list of properties and params, I proceed to use the DOM methods to build my Object and Param tags. Well, [Prototype](http://prototypejs.org) ’s Element methods, actually. It worked in Firefox, and partially in Safari / Chrome, and none at all in Internet Explorer 7.

So then, a little research shows that all the Javascript solutions for inserting a video object doesn’t use Element at all. It would seem that extends the HTMLObjectElement cause problems. The solution: _simply write the HTML into a string, and set the innerHTML of an Element to your object._

Here’s how Mootools handles it in its [Swiff class](http://mootools.net/docs/Utilities/Swiff) :

    var build = '<object id="' + id + '"';for (var property in properties) build += ' ' + property + '="' + properties[property] + '"';build += '>';
    for (var param in params){    
    	if (params[param]) 
    		build += '<param name="' + param + '" value="' + params[param] + '" />';
    }
    build += '</object>';

