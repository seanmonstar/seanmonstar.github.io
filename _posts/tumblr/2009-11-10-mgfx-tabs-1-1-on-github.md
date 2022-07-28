---
layout: post
title: MGFX.Tabs 1.1 on Github
date: '2009-11-10T08:13:00-08:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/709042849/mgfx-tabs-1-1-on-github
---
Quite a while ago, I released a simple-to-use [MooTools tabs class](http://seanmonstar.com/2022/07/28/2008-08-12-pluggable-mootools-tabs.html), and it continues to be one of the most frequented posts on the site. With so many people obviously desiring a MooTools tabs plugin, I’ve added a few new features to the plugin, as well as moved it to Github for inclusion in the future MooTools Forge.

[<figure class="tmblr-full" data-orig-height="85" data-orig-width="375"><img alt="Pluggable MooTools Tabs" src="https://64.media.tumblr.com/3146a6a25457aa6b4eb1ddf3c1fb0d5e/dd255328748b07bf-30/s540x810/bfe0b779c0b453cabeccf3e4afbacef1c140ac99.jpg" data-orig-height="85" data-orig-width="375"></figure>](http://github.com/seanmonstar/MGFX.Tabs)

#### On Github

First of all, [all development on this class will be found in Github](http://github.com/seanmonstar/MGFX.Tabs) from now on. That means you can always find the most recent work on it there, and can even fork or provide improvements if you ever so desire. When I add new features, I’ll add a new tag grouping them together and allowing an easy way to download.

With putting it on Github, I’ve also organized the project into various folders, provided some basic documentation, and done the rest that I need to in order to eventually include this in the [MooTools Forge](http://mootools.net/blog/2009/09/22/mootools-124-released/). Seeing as this class is one of the popular MooTools tabs plugins, I imagine it will be a good fit on the Forge.

##### Demo and Docs

There’s [documentation](http://github.com/seanmonstar/MGFX.Tabs/blob/master/Docs/mgfxtabs.md)&nbsp;that I wrote up explaining inner workings of the class. Plugins for the Forge are requested to have documentation. Also, a [demo](http://seanmonstar.github.com/MGFX.Tabs/) showing off the newest options is on Github as well.

#### Version 1.1

On Github, the [newest version as of this writing is 1.1 <ins>1.1.1</ins>](http://github.com/seanmonstar/MGFX.Tabs/tree/1.1.1). [1.0](http://seanmonstar.com/2022/07/28/2008-08-12-pluggable-mootools-tabs.html) was the initial release, and since then people left in the comments some good suggestions to improve the class. I had first simply included the suggestions haphazardly, but soon realized I should do it properly.

##### New Options

Specifically, new in version 1.1 is 2 new options when instantiating the class. There’s `hover` and `hash`.

`hover`, which I plugged with a hack originally, determines if hovering over the Tabs area should pause the autoplay animation. Before it simply called `stop` and `autoplay`, but it was suggested that it looked weird if the timer went 4 seconds, then you hovered over the tabs, and then after leaving the area it took its allotted 5 seconds over again. So now, I do some timer magic to make sure that each slide is only shown for as long as you set in `slideInterval` (not counting while the mouse is over the area).

`hash` is another option that was requested, that makes the plugin take a look at `window.location.hash` to determine if there is a Tab to show. If you go to `#some-id`, and one of the tabs has the id “some-id”, then that will be the first tab to be shown.

##### Doing Things Differently

I also broke it slightly. I mean it’s not very backwards compatible. The constructor takes arguments _every so slightly differently_. I used to require that the buttons be in an unordered list. I felt that requirement was silly of me. So I’ve removed that need. Now, you simply provide a collection of elements that should be the buttons. To still use an unordered list, it’s as simple as:

    new MGFX.Tabs($$('ul.tabs li a'), panels);

But you can also make it more complicated, if you need to:

    new MGFX.Tabs($$('li.one a, li.two a, #extra .button'), panels);

The same can be done to the panels as well. I saw some people trying to make a different element that came from a completely different part in the HTML, still be used in the Tabs. I figured this would make it easier on people.

#### Tabs in the Future

When the MooTools Forge comes out, I’ll be adding the most recent tag to the Forge. Hopefully, when I get the time to do some more, I’ll add in more transition effects, as well as the ability to load some content via Ajax into the tab panels.

Here’s a courtesy link to the zip on Github.

[MGFX.Tabs 1.2.0](http://github.com/seanmonstar/MGFX.Tabs/zipball/1.2.0.zip)

#### Update (11/18/09)

Couple bug fixes were made, one with the new hover functionality wrongly restarting the animation, and another that broke the tabs when you clicked the tabs too quickly. I’ve updated the links to version 1.1.1.

#### Update (2/18/10)

MGFX.Tabs is now at [version 1.2.0](http://seanmonstar.com/2022/07/28/2010-02-18-mgfx-tabs-1-2-0-show-a-random-tab.html). There is now a `MGFX.Tabs#random` function that can be used to display a random tab. Ta-da!

