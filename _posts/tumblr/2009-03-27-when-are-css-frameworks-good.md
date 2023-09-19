---
layout: post
title: When Are CSS Frameworks Good
date: '2009-03-27T09:01:00-04:00'
tags:
- css
- standards
- opinion
tumblr_url: https://seanmonstar.com/post/707141572/when-are-css-frameworks-good
---
There are loads of CSS frameworks out there, and plenty more of articles telling you that _they are the devil and don’t use them_. I agree with these articles as a whole. So what am I trying to say? Frameworks can be good? It depends.

#### Make You Own

Don’t go and grab any of the gillions of CSS frameworks out there. No, this post is actually about **your own CSS framework**. Personally, as I’ve sliced a multitude of designs, there’s certain class names and ID’s I like to use. I have a preference on the semantic mark-up for a set of elements. And most web-sites are structured the same, with slight variations.

It’s therefore, quite easy to keep a base CSS file, which I call framework.css, that already has done the things I do every single design. Things like floating 2 or 3 columns together, giving proper padding and margin that I always use.

![framework css](http://monstar.blazonco.com/images/blog/framework.png)

#### Style as Generally as Possible

Something else that makes your own CSS really powerful, is trying to use **classes** (when you need them) **to identify elements**. And also, styling elements with **the least specific rules you can manage**. This way, when new html is inserted (by the client, or even yourself) your CSS hopefully already applies. If you made a box that goes on the left, and styled it pertaining to it’s ID, what happens when you add a new box underneath? Were you (or the client) hoping it would be just like the box above it? How much better it would have been if you had been less specific.

Have you built up your own framework that you continue to use?

