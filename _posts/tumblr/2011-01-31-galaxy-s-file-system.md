---
layout: post
title: Fixing the Galaxy S File System
date: '2011-01-31T22:31:14-08:00'
tags:
- android
- samsung
- galaxy s
- lag-fix
tumblr_url: https://seanmonstar.com/post/3044912970/galaxy-s-file-system
---
My [phone](http://www.amazon.com/gp/product/B003TXSKNE?tag=seanmonstar-20) felt pretty fast when [I got my hands on it](http://seanmonstar.com/2022/07/28/2010-09-29-samsung-galaxy-vibrant-review.html) last summer, but I’d done my fair share of complaining about seemingly super slow apps. I figured the developers of those apps just plain sucked. I was a little surprised that [my brother](http://tractorbeamtuesdays.tumblr.com), owner of a Droid X, didn’t notice the same slow downs. A [Google search later](http://androidforums.com/captivate-support-troubleshooting/215225-wondering-why-facebook-app-really-slow.html), and I found out it was actually Samsung who sucks. Samsung built a rockin’ phone, and included a terrible [Robust FAT File System](http://www.samsung.com/global/business/semiconductor/products/fusionmemory/Products_RFS_Brochure.html)<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>.

The fix is to convert to an [Ext4](http://en.wikipedia.org/wiki/Ext4) file system. Theoretically, you just flash a patch that changes your file structure, but I believe I picked a patch that was incompatible with the ROM I originally had installed. My phone would no longer boot, and I cried. If this happens to you, feel free to cry. Then, follow [this guy’s video walkthrough](http://www.youtube.com/watch?v=9nMCBbdAcHU), and your phone is back to factory settings.

Personally, I figured that since I was already going to root and flashing something to my phone again, I might as well pick a ROM that was newer and had improvements gleaned from the new Gingerbread source code. Thankfully, [Team Whiskey](http://www.teamwhiskey.com/DownVibrant.html) has been making quite an awesome ROM that uses the current voodoo lag fix, and is as close to Gingerbread as you can get, until someone successfully ports all of 2.3 to the Galaxy S.

![](https://64.media.tumblr.com/tumblr_lfx8yxmLNl1qzek7l.png)

To do this sort of thing quickly, follow these steps.

1. [Root your device](http://wiki.cyanogenmod.com/index.php?title=Samsung_Vibrant:_Rooting).
2. Install Clockwork Recovery ROM. Can be found in the Market. When it reboots, if you see the standard recovery rom (which is blue), simply select “reinstall packages”, and you should see it reboot into the green, Clockwork recovery from now on.
3. Install [Bionix-5](http://www.teamwhiskey.com/DownVibrant.html) (Samsung Vibrant only).
4. After Bionix-5 finishes, reboot the phone 10 minutes later just for good measure, cleaning everything up.
5. Marvel at the insane speeds of your phone, now that it isn’t encumbered by a terrible file system.

It seems complicated at first, but once you start the flashing, you’ll be surprised at how much of it does itself. While you’re at it, if you want any other goodies, or perhaps a different ROM, checkout the [Vibrant bible](http://forum.xda-developers.com/showthread.php?t=771111) over on XDA.

* * *

1. 

If you’ve noticed things like “lag-fix”, or “voodoo” fix, those are all about how to fix Samsung’s screw up.&nbsp;[↩︎](#fnref:1)

