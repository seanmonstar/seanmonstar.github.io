---
layout: post
title: At Least Pad the Default Button Styles
date: '2010-06-08T10:12:00-07:00'
tags:
- css
- usability
- ui
tumblr_url: https://seanmonstar.com/post/709076819/at-least-pad-the-default-button-styles
---
Some people like to [style buttons to their own design](http://seanmonstar.com/2022/07/28/2009-04-29-use-css-borders-for-3d-effects.html), ensuring they look the same cross browser or enhance the site theme. Others are perfectly fine with a `<button />` being rendered differently depending on browser and operating system combination. I understand both sides. Sometimes a styled button can look really nice. Other times, it just makes sense to leave it at default, because users are used to seeing buttons the way their OS always displays them.

As long as buttons have a consistent look across the site, I’d say both are just as usable. If you’re in the latter camp, leaving button styles to the browser, please, oh please, at least **add some padding to the buttons**. The default button styles leave the border right on top of the text, making it very annoying to read the text of the button.

<figure class="tmblr-full" data-orig-height="74" data-orig-width="344"><img src="https://64.media.tumblr.com/869d61154dcd9952168f895de25420af/d9aa98bea7cf9f44-e4/s540x810/f9ca1c5d7a20e6152c13bd566d5016de7ebbc251.png" data-orig-height="74" data-orig-width="344"></figure>

Here’s a screenshot from [Remember the Milk](http://www.rememberthemilk.com/) on the left, and my addition of padding on the right.

    input[type="submit"], input[type="button"], button {
    	padding: 3px 5px;
    }

Just a few pixels, but the difference is _huge_. You can now read the text of each button with ease. **Please, [don’t forget the whitespace](http://www.usabilitypost.com/2010/06/04/dont-forget-the-whitespace/)**.

