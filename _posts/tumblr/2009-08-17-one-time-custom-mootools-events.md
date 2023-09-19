---
layout: post
title: One-Time Custom MooTools Events
date: '2009-08-17T11:19:00-04:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/707187636/one-time-custom-mootools-events
---
MooTools lets you listen to events from 2 mediums: Elements and Classes. From classes, you can fire any event you want, and listen for it elsewhere. But with Elements, events are usually tied towards native DOM events. MooTools gives you the ability to define custom events for elements, by adding an entry to `Element.Events` .

Some practical uses can be found, such as the domready event that the document fires, or extending the [click event to always change your cursor](http://davidwalsh.name/mootools-add-event). I want to touch on this interface though, to show how you might solve my problem I mentioned 2 weeks ago: looking for [events that would only fire once](http://mcarthurgfx.com/blog/article/one-time-events-already-fired).

This is only useful for Element Events. I haven’t researched doing this with an event from a class. That will probably use the same principle. But let’s move on!

#### Element.Events

When you add an event to `Element.Events` , you specify an object with up to 4 properties:

- `base` : A DOM Event that is needed to trigger this. For instance, a doubleclick event would use `click` as it’s base.
- `condition` : a function the returns a boolean for whether the event should fire. Our same example, doubleclick, would time the interval between clicks and return true if the time difference was small enough.
- `onAdd`: a function that gets called whenever you subscribe to the event somewhere else ( `someEl.addEvent('doubleclick', ... )` ). 
- `onRemove` : a function the gets called when you called `removeEvent` 

With this in mind, we will consolidate the process of keeping track of **one-time events** in our custom event.

#### oneClick

> Attach a listener, determine if the event has already fired, and if so, execute the listener immediately.

I couldn’t come up with an excellent custom event, so we’ll pretend that I might have an element that can clicked once, and only once, no matter what. Let’s implement oneClick.

Every time I subscribe to `oneClick` , we’ll check a property to see if it’s fired before, and if so, execute our new listener then and there. If not, continue as normal. Then, when the event does fire, we will set a property that the event has fired in the condition, and after that, make our condition return false, so that the base event never makes our custom event trigger more than once.

     Element.Events.oneClick = {
    	base: 'click',
    	condition: function(event) {
    		var fire = !this._oneClicked;
    		this._oneClicked = true;
    		return fire;
    	},
    	onAdd: function(fn) {
    		if(this._oneClicked) {
    		fn.call(this);
    	}
    }};

Now, we can listen on an element for oneClick, and listen for it somewhere else, perhaps later. Who cares. I can feel assured that this event will happen only once. And all new listeners that might have depended on my element being clicked (like a Start button) will work too!

    $('startButton').addEvent('oneClick', function() {
    	Timer.start();
    });
    // ... else where
    $('startButton').addEvent('oneClick', function() {
    	Toolbar.show();
    });

##### One Limitation

So far, the **one limitation** I found, was that you had to not need the actualy Event object in your listener. As in, you can’t check the `event.target`, or mouse x and y coordinates from the event. This is because from the onAdd method, we don’t have access to a native event, since one hasn’t actually fired. I found this trying to call `e.stop()` in my testing script. This doesn’t bother me much, but be warned.

Implementing this for Elements was easier than for Classes, because of MooTools custom element events system that already exists. I have an actual need for the Class events to be one-time, but found this out along the way. **Do you have a use for this?**

