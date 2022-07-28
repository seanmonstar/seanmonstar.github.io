---
layout: post
title: Make YUI's Editor More Pluggable
date: '2008-09-03T13:57:00-07:00'
tags:
- javascript
- yui
tumblr_url: https://seanmonstar.com/post/706983762/make-yui-s-editor-more-pluggable
---
I got to play around a little bit with YUI’s (I pronounce it _yoo-ey_) [Rich Text Editor](http://developer.yahoo.com/yui/editor/ "YUI Editor") as I implemented a low-level CMS for one of my current projects. I’ve used TinyMCE a-plenty, and wanted to check out YUI’s implementation because theoretically _YUI is far easier to extend._ One thing that tripped me up was the styling, which in retrospect I should have noticed, but it irked me, so I solved it in my lazy efficient programmer fashion.

After ripping the stylesheet and scripts straight out of the YUI demo, and start up my page, you notice that the Editor loads in, but it looks way wacked. That’s because I forgot the subtle declaration of a new class on the body element:

Once finally catching this, I felt disappointed that the YUI Editor didn’t do that in the Editor.render() function that you call to display the editor. My opinion of plug-ins is that they should be as pluggable as possible. _The less I have to do to make a plug-in work, the better the plug-in, in my opinion._

Having to add a class name in HTML that’s _just as easily done in Javascript_ didn’t make sense. If Javascript wouldn’t is disabled and isn’t able to add the class to the document body, then the class is useless anyways, since the YUI Editor won’t be using it.

    var myEditor = new YAHOO.widget.Editor('msgpost', { height: '300px', width: '522px', dompath: true, animate: true }); myEditor.render(); YAHOO.util.Event.onDOMReady(function(){ YAHOO.util.Dom.addClass(document.body,'yui-skin-sam'); });

The last line uses YUI’s onDOMReady function to wait until the DOM is accessible, then uses YUI’s Dom object to add `yui-skin-sam`to the body tag.

This may seem like a useless modification, but now, I can simply copy and paste this code snippet, or even externalize and just include the Javascript file, into any page I want, and not have to remember to add a class to my HTML.

