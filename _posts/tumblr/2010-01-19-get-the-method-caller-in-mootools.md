---
layout: post
title: Get the Method Caller in MooTools
date: '2010-01-19T12:48:00-05:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/708933763/get-the-method-caller-in-mootools
---
As I continue to work on my MVC implementation in MooTools, I continue to find new hidden features in MooTools. This weekend, I was adding a view method to the controller, as a shortcut to creating a new View and rendering it. One of the arguments is the view file name, but I also wanted some automagic like CakePHP. It’d be great if the view function could determine the file name based on the function that called it.

Here’s what I mean:

    var ItemsController = new Class({
    	view: function(view_name) {
    		if(!view_name) {
    			//viewname should default to 'controller/method'
    		}
    	},
    	list: function() {
    		$(this).grab(this.view()); // 'items/list'
    	}
    });

Every method in a MooTools Class is wrapped in some decorators, that set the caller on the function object when called. By staring and counting how many methods back I needed to collect, I’ve accomplished my goal. However, **this won’t work if a standard function calls this method**. It might be a good idea to [protect this method](http://seanmonstar.com/blog/2009-09-04-protected-methods-in-mootools-classes/), now that I think about it.

    //inside view function
    view_name = arguments.callee.caller.caller.caller._name;

Don’t believe me? Test it right here. As neat as this is, _I don’t gaurantee this will work in every situation_, and I haven’t tested if any browsers break using this.

<iframe style="width: 100%; height: 300px" src="http://mootools.net/shell/9QDKe/embedded/"></iframe>
