---
layout: post
title: Use CSS Borders for 3D Effects
date: '2009-04-29T11:55:00-04:00'
tags:
- css
- design
- bestof
tumblr_url: https://seanmonstar.com/post/707101490/use-css-borders-for-3d-effects
---
Recently, I’ve been more in favor of using the browsers rendering instead of background images to achieve looks, since that **reduces bandwidth and loading times** , and also **increases maintainability**. I used to slice images that would give something a shadow or button type of a look, but have moved more towards just making the browser do it. I wanted to share some examples of effects and how easy it is to do so.

#### 3D Buttons

As a smaller example, I’ll show the buttons I put together for the Blazonco admin post index page. Rather simple, but I wanted to achieve that effect you’ve seen in older buttons. You know, where the top part is light than the bottom, so it looks slightly raised out of the application. And then on hover, I swap the border colors, giving the look of being pushed in.

<figure data-orig-height="54" data-orig-width="223"><img src="https://64.media.tumblr.com/184349bf94ef6138e904cce0d8152f12/f3fc7df99603f497-b9/s540x810/98da4aee1756f5625830aa389dcaccb858033bf7.png" data-orig-height="54" data-orig-width="223"></figure>

I threw a darker color on the bottom and right sides. The above picture shows normal (left) and hover (right). Of course, Print Screen hides the cursor (which I found weird). And here’s the relevant CSS:

     a.button {
    
        background-color:#fff;
    
        border-width:1px;
    
        border-style:solid;
    
        border-bottom-color:#aaa;
    
        border-right-color:#aaa;
    
        border-top-color:#ddd;
    
        border-left-color:#ddd;
    
        border-radius:3px;
    
        -moz-border-radius:3px;
    
        -webkit-border-radius:3px;
    
    }

_Having it in CSS allowed me to easily test out the results_ , and tweak the color slightly as I decided one side was too dark or too light. And I also was able to easily experiment with how round I wanted the button. When using images for all this, you’d have to make the changes in Photoshop, and save it and upload it again. So a major gain is in **how easy it is to maintain**.

#### Shadows

I had the joy of turning a comp with mutliple rounded corner boxes and shadows and the like into a working web-site not long ago. I originally thought that these boxes would also need to be expandable, and I wondered how to achieve this in a feasible way. I settled on using various CSS3 techniques instead of image slices with [sliding door](http://www.alistapart.com/articles/slidingdoors/) techniques.

The end result was as such:

<figure class="tmblr-full" data-orig-height="206" data-orig-width="415"><img src="https://64.media.tumblr.com/88bfd25c2e8ae4922066da63b9dc7958/f3fc7df99603f497-05/s540x810/09b6cadd871d8a676e181a26ea444b2f0df20927.png" data-orig-height="206" data-orig-width="415"></figure>

In the boxes, I used a gradiant slice for the background, and then used only border and border-radius to obtain a kind of 3D look. Here’s the CSS:

     .module {
    
        background:#fcfcfc url('images/module.png') repeat-x left bottom;
    
        border:1px solid #ccc;
    
        border-bottom:2px solid #bbb;
    
        border-top:1px solid #ddd;
    
        border-radius:10px;
    
        -webkit-border-radius:10px;
    
        -moz-border-radius:10px;
    
    }

By making the border bigger and darker on the bottom, and lighter and thinner on the sides and top, I gave it a pseudo-shadow. The benefits of this over an image is that it only took 1 element to style (the div), and it could easily expand in any direction without running out of image.

Since it was using CSS3 techniques, it was squared in Internet Explorer, but continued to look good. This is an important factor of [progressive enchancement](http://www.alistapart.com/articles/progressiveenhancementwithcss) , since I made sure to use CSS that looked nice in older browsers.

##### CSS Feature Detection?

One last rule I would have liked to add but never did was using Safari’s drop shadow property. That would have achieved an even nicer looking shadow in Safari. The reason holding me back, was that I’d already used border’s to accomplish this effect, and didn’t want to try to remove them for Safari. This would have required some Safari CSS hacks, since I currently don’t know of a way to do CSS feature detection.

It’d be cool if you could do something as such (I’m going to steal some Javascript syntax):

     if(-webkit-box-shadow) {
    
        .module {
    
            border:none;
    
            -webkit-box-shadow:5px 2px 2px #bbb;
    
        }
    
    }

Do any of you have any suggestions on how to properly achieve that with current technologies? And I’m not interested in server-side sniffing. That’s obvious, and sub-optimal.

