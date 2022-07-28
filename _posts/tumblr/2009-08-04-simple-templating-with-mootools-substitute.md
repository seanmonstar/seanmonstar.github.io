---
layout: post
title: Simple Templating with MooTools Substitute
date: '2009-08-04T09:00:00-07:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/707177494/simple-templating-with-mootools-substitute
---
As I continue to flesh out my MooTools MVC framework, I found a neat way to implement a **templating system** , using only what MooTools gives me. The goal of templating systems is to allow you to write in your target format (usually HTML), and denote where variables should be tied in.

MooTools gives this power directly to every String!

#### String.prototype.substitute

[Substitute](http://mootools.net/docs/core/Native/String#String:substitute) lets you write a string, declaring delimiters where you would like variables to belong. So we can write:

    var example = 'My name is {name}. I love {passion}';

Now, all we have to do is use an object with those properties set, and call the substitute function, and it will do all the replace [magic](http://mcarthurgfx.com/blog/article/job-title-web-magician) for us.

    var obj = { 
    	name: 'Sean', 
    	passion: 'programming' 
    };
    var subbed = example.substitute(obj);
    alert(subbed); //My name is Sean. I love programming';

Now, I said we can use this awesome power for a simple template usage. See where this is going?

#### A Simple Template View

My view has more going on than this, but I’ll remove the rest so you can see simply the templating part that is going on. It’s very easy to use, indeed!

    var View = new Class({        
    	template: '<a href="/task/view/{id}">{name}</a>',        
    	render: function(data) {        
    		var template = this.template.substitute(data);        
    		return new Element('div',{html: template}).getFirst();    
    	}    
    }); 
    var firstTask = {    
    	id: 1,    
    	name:'Write Subsitute Article'
    };
    var taskView = new View();
    document.grab(taskView.render(firstTask));

By defining the template in a string, we just write out the html for an anchor tag of my view. We make a new View, make an object, and tell the view to render using the object. The last part is simply turning the string into HTML by inserting it into a div, and then grabbing the element back out.

