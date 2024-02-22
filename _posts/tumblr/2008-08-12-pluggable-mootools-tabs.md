---
layout: post
title: Pluggable Mootools Tabs
date: '2008-08-12T16:54:00-04:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/707019146/pluggable-mootools-tabs
---
#### Update (2/18/10)

Theres a [new version of MGFX.Tabs](http://seanmonstar.com/blog/mgfx-tabs-1-2-0-show-a-random-tab/), that includes the ability to easily show a random tab.

#### Update (11/10/09)

I’ve released a [new version of MGFX.Tabs](http://seanmonstar.com/blog/mgfx-tabs-1-1-on-github/), and wrote a new article explaining the new features, its new location, and so forth.

---

<figure class="tmblr-full" data-orig-height="85" data-orig-width="375"><img src="https://64.media.tumblr.com/3146a6a25457aa6b4eb1ddf3c1fb0d5e/58f2924e13d99964-c4/s540x810/30ab7de8ac4daf8c2c2dc8a8f2af5bad16918ec2.jpg" data-orig-height="85" data-orig-width="375"></figure>

Last week, I released a pluggable Slideshow type Mootools class. I had written that class a while back, and since then had extended its functionality to allow me easily make tabs on any page. I wanted my tabs to have the ability to auto-switch if I wanted, so extending my Rotater class made perfect sense.

#### MGFX.Tabs

I also wanted this to be a solution that _didn’t require going into the source_ and tweeking it to make it work. That’s the point of every class, really.

Also, people (even myself, on varying projects) use different mark-up to make tabs. The placement in the mark-up doesn’t matter—just know that _it associates controller links with content based on order_ in the DOM. I often use a `ul` of controlling links, and then `div`s to hold the content; but sometimes I make the content an unordered list also. So this class tries not to restrict the xHTML, barring one: it does assume your controls are links inside list items (`<li><a></a> </li>` ).

This does require the Fx.Elements add-on on top of the regular Mootools framework, so be sure to grab the extra class from [Mootools More](http://mootools.net/more) .

##### The Constructor

The _constructor_ is pretty simple, and since it extends Rotater, it includes the same transitionDuration and slideInterval options.

    window.addEvent('domready',function(){ var tabs = new MGFX.Tabs('.tabs','.content',{ autoplay: true, transitionDuration:500, slideInterval:6000 }); });

The _first parameter_ taken is the class name of `ul` that contains your controlling buttons/links. The _second parameter_ is the class name of all the tabbed cotent. For both of these, simply make the proper elements have the same class name.

The _third parameter_ is optional, as it is the class options. Every option can be ommited if you want, and it will just use the defaults I felt would be common.

**Namespace:** I’ve started prepending all of my classes with _MGFX_ to prevent conflicts with any other code that might be in use.

[MGFX.Tabs 1.2.0](http://github.com/seanmonstar/MGFX.Tabs/zipball/1.2.0.zip)

There ya have it! Should be fairly straight foward. _The demo is now in the download_, so it will forever work, and won’t break when I alter my own Javascript.

