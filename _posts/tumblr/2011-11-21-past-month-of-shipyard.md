---
layout: post
title: Past Month of Shipyard
date: '2011-11-21T14:45:32-05:00'
tags:
- javascript
- mvc
- shipyard
- planet
- mozilla
tumblr_url: https://seanmonstar.com/post/13120286940/past-month-of-shipyard
---
Last month I got to start working on [Shipyard](https://github.com/seanmonstar/Shipyard) almost full-time, as I need it to make [Add-on Builder](https://builder.addons.mozilla.org) work by the end of the quarter. I’m not ready to call Shipyard 1.0 until I’m confident with the API for Models, Controllers, and Views. Models are largely done, Views need some work, and [Controllers need a start](https://github.com/seanmonstar/Shipyard/issues/1). Besides that, though, what has happened to Shipyard in the past month?

Here follows a changelog-ish list.

- Animations
  - A merge between MooTools’ Fx.Tween and Fx.Morph classes
- Events
  - `addListener` returns a Listener object with `attach` and `detach` methods.
  - support for `once`
  - legacy API
    - official API for EventEmitters is `addListener`, `removeListener`, and `emit`
    - previous methods will soon log a DeprecationWarning, before being removed by 1.0
- New utils
  - Logger
    - Makes the console more familiar for Python users
    - Doesn’t error if there is no console available
  - Cookie
    - Thanks to MooTools
  - Color
    - Thanks to MooTools
- DOMEvent
  - Thanks to MooTools
- Env
  - provides `browser` and `platform`
- A briefer test runner
  - Only prints dots for successes, F for failures, and prints errors at the end

If commit lists are more your thing, you can [look to GitHub](https://github.com/seanmonstar/Shipyard/compare/14ed52f...ab0f4bd7). The next month should see event delegation, and more work on the [View/template system](https://github.com/seanmonstar/Shipyard/issues/2).

