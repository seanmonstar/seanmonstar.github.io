---
layout: post
title: MGFX.Tabs 1.2.0 - Show a Random Tab
date: '2010-02-18T10:52:00-08:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/709053684/mgfx-tabs-1-2-0-show-a-random-tab
---
[<figure class="tmblr-full" data-orig-height="85" data-orig-width="375"><img src="https://64.media.tumblr.com/3146a6a25457aa6b4eb1ddf3c1fb0d5e/a33eff24964139ab-8d/s540x810/4d6f9e9b8d9c547eae5e5a4162fd76f6906a6078.jpg" alt="MGFX.Tabs" data-orig-height="85" data-orig-width="375"></figure>](http://mootools.net/forge/p/mgfx_tabs)

I’ve updated MGFX.Tabs to have a random slide function. It just uses `Math.random`, and a slight modifier to insure the random number isn’t already the current index. Since I believe in [semantic versioning](http://semver.org/), and this is a feature update (not a bug fix), but not breaking, its should be a new minor version.

    var tabs = new MGFX.Tabs($$('.tabs li a'), $$('.tabs .content'));tabs.random();

**So [MGFX.Tabs 1.2.0](http://mootools.net/forge/p/mgfx_tabs) is available on the Forge.**

