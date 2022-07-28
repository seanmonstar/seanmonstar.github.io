---
layout: post
title: Image Replacement and Backgrounds
date: '2008-06-17T10:26:00-07:00'
tags:
- css
tumblr_url: https://seanmonstar.com/post/706943372/image-replacement-and-backgrounds
---
Having been around the CSS playground for a while now, I’ve seen the different techniques of **image replacement** , and I’d like to share my methods of doing so, and why I do so.

#### Heading Replacement

Sometimes, we really love a font or layer style so much, we decide we just have to make our headings to be images. Most commonly, this includes the H1 of the web-site, the logo. Considering the logo has a distinct and perfect look to it, quite rightfully it should usually be an image.

##### The Spanning Pool

There’s the handy IMG tag for us to use, but when thinking semantically, it’s better off keeping headings in heading tags. The goal is to give text-readers the proper text inside the heading, but for modern browsers to show the image in all its glory. It seems quite common, though, to keep a SPAN tag inside the heading to achieve the image replacement. Let me first show you what **not** to do.

    \<h1\>\<span\>McArthur GFX\</span\>\</h1\>

    h1 { background:url('images/logo.jpg') no-repeat; height:100px; width:300px;}span { **display:none; /\* wtf mates? \*/** }

This method does accomplish the proper aesthetic result, but _has a few imperfections_.

**Firstly** , we have an extra span tag inside the H1 tag. Not too big of deal. But it’s unnecessary, and therefore improper. **Secondly** , making the text have the property display:none may actually affect some text readers to the point that the _all-important heading_ is **ignored!**

##### Text-Indent

Conversely, we can use the CSS property _text-indent_ to accomplish the same aesthetic goal, and leave the text in its proper display.

    \<h1\>McArthur GFX\</h1\>

    h1 { background:url('images/logo.jpg') no-repeat; height:100px; **text-indent:-9999px; /\* the bbq sauce \*/** width:300px;}

This essentially throws the text 10,000 pixels to the left, which is of course larger than any monitor (currently… wow that’ll be one big sucker). But when text readers look through the HTML, the text-indent is considered an entirely aesthetic change, and won’t affect them.

#### Partial Image Replacements

There are plenty of occasions where we can create a background of an element with a nice gradiant or pattern and want it in the live version. But there’s going to be lots of elements using the same background. So we’ll use a similar technique as above, but just leave out the text indent.

    \<ul id="nav"\> \<li\>\<a href=""\>A Cool Link\</a\>\<li\> \<li\>\<a href=""\>A Cold Link\</a\>\<li\> \<li\>\<a href=""\>A Warm Link\</a\>\<li\> \<li\>\<a href=""\>A Hawt Link\</a\>\<li\>\</ul\>

It’d stink to make the image and save it for every single instance with different text. So we just save the background, apply the background, and keep the text over it, so now we can change the text whenever we want. It can say anything too!

    #nav li { background:('image/bg\_link.jpg') no-repeat; height:20px; width:80px;}

This will give all of the changeable links in #NAV to have the same button image. Great for adding new links, and changing old ones. And great on bandwidth and loading time, because now the visitor only needs to download one image to have the background for all.

##### The Pattern / Gradiant

What if your links simply have gradiant, or pattern of scan lines or something similar? We can make one teeny change to our CSS, and reduce bandwidth and loading even more!

    #nav li { background:('image/bg\_link.jpg') **repeat-x; /\* the fast lane \*/** height:20px; width:80px;}

Save the gradiant as 1px by Xpx, depending on how long the gradiant changes. Or smallest dimension possible to contain the repeating pattern. With the dimension of 1px x 100px, that is massively smaller in file size than 100px by 100px, for example.

**An added benefit** to a repeating background is having the possibility of variable width. With a fixed width background for buttons, we must keep the text under a certain length or expect _certain aesthetic doom_. With a repeating background, you can remove the WIDTH property and let the buttons have a natural length based on the text.

