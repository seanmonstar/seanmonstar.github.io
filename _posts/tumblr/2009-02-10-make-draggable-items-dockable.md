---
layout: post
title: Make Draggable Items Dockable
date: '2009-02-10T07:05:00-08:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/707093656/make-draggable-items-dockable
---
I’ve been working on a RTE using Mootools, and I wanted the toolbar to draggable. I always like it when I can drag something to the edge and it will dock itself. So, I extended [Drag.Move](http://mootools.net/more) to allow docking: Drag.Dock. With this class, which requires Drag and Drag.Move from the Mootools More, the draggable elements can now be docked to any side of the window.

    Drag.Dock = new Class({
    	Extends: Drag.Move,
    	options: {
    		proximity: 20
    	},
    	initialize: function(element, options, location) {
    		$(element).setStyle('position','fixed');
    		this.setOptions(options);
    		this.parent(element, this.options);
    		this.dock(location || 'top');
    	},
    	drag: function(event) {
    		this.parent(event);
    		var windowHeight = window.innerHeight || document.documentElement.clientHeight;
    		var windowWidth = window.innerWidth || document.documentElement.clientWidth;
    		if(this.element.offsetTop < this.options.proximity) {
    			this.dock('top');
    		}
    		if(this.element.offsetTop + this.element.offsetHeight > windowHeight - this.options.proximity) {
    			this.dock('bottom');
    		}
    		if(this.element.offsetLeft < this.options.proximity) {
    			this.dock('left');
    		}
    		if(this.element.offsetLeft + this.element.offsetWidth > windowWidth - this.options.proximity) {
    			this.dock('right');
    		}
    	},
    	dock: function(location) {
    		var windowHeight = window.innerHeight || document.documentElement.clientHeight;
    		var windowWidth = window.innerWidth || document.documentElement.clientWidth;
    		switch(location) {
    			case 'top':
    				this.element.setStyle('top',0);
    				break;
    			case 'bottom':
    				this.element.setStyle('top',windowHeight - this.element.offsetHeight);
    				break;
    			case 'left':
    				this.element.setStyle('left',0);
    				break;
    			case 'right':
    				this.element.setStyle('left',windowWidth - this.element.offsetWidth)
    				break;
    		}
    	}
    });

The options for Drag.Dock are of course all that would be applied to [Drag.Move](http://mootools.net/docs/Plugins/Drag.Move) , plus `proximity` , which defaults 20. This is how many pixels away from the screen edge before it tries to dock to the edge.

And the convenience method, similar to [makeDraggable](http://mootools.net/docs/Plugins/Drag.Move#Element:makeDraggable) :

    Element.implement({
    	makeDockable: function(options,location) {
    		return new Drag.Dock(this,options,location);
    	}
    });

Grab&nbsp;[drag-dock.zip](http://mootools.net/forge/p/drag_dock)&nbsp;from the Forge.

