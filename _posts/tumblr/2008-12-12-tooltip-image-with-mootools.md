---
layout: post
title: Tooltip Image with Mootools
date: '2008-12-12T12:29:00-05:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/708849612/tooltip-image-with-mootools
---
<figure class="tmblr-full" data-orig-height="200" data-orig-width="300"><img src="https://64.media.tumblr.com/daf6f7bba9c26831b96a052c1ab5a0ed/36550a15c6504406-4c/s540x810/937e7bbdf6ce6f1a2e9bddb4afe58178dc55779c.jpg" data-orig-height="200" data-orig-width="300"></figure>

Alright, taking a look at the Mootools docs, and playing around, I’ve quickly learnt how to throw an image into a Tooltip with Mootools. It’s far easier, and far cleaner than what I was doing with version 1.1. The first way still works, and the validator tells me its valid, but it feels dirty.

**A little background:** the tooltip is built from the title and rel (or href if rel is empty) attributes of the element.

#### Putting \<img\> in the rel property

    window.addEvent('domready',function() { var t = new Tips('.tip'); });

    \<a class="tip" href="http://mootools.net/" rel="&lt;img src=mootools\_white.jpg /&gt;" title="Mootools.net"\>Mootools\</a\>

This was my original, dirty way. It should still work. Make sure to escape the left and right brackets inside the rel property, or you’ve got invalid XHTML. So if that looks like your bag of goodies, take it. For those who like feeling good inside, here’s a newer, cleaner way of doing things.

#### Using Element.store to store an image

The Mootools Tips class use [Element.store](http://mootools.net/docs/Element/Element/#Element:store) and [Element.retrieve](http://mootools.net/docs/Element/Element/#Element:retrieve) for the tool-tips title and text, so that means we can override the defaults. It just requires a little more scripting, but I think that’s perfectly fine.

The benefit here is that you can make a new DOM Element of an image, and store the image into the tip:text field of the Element. Take a peek:

    window.addEvent('domready',function() { var t = new Tips('.tip'); $$('.tip').each(function(tip){ var imgSrc = tip.retrieve('tip:text'); var imgAlt = tip.retrieve('tip:title'); tip.store('tip:text', new Element('img',{'src':imgSrc,'alt':imgAlt})); }); });

This way, we can store the filename of the image in the rel property, and then construct the image with Javascript. Feels a whole lot cleaner than keeping an escaped img tag inside a property.

    \<a class="tip" href="#" rel="mootools\_white.jpg" title="Mootools.net"\>Mootools\</a\>

