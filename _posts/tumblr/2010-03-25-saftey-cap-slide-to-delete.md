---
layout: post
title: 'Saftey Cap: Slide to Delete'
date: '2010-03-25T08:40:00-07:00'
tags:
- javascript
- ui
tumblr_url: https://seanmonstar.com/post/709020436/saftey-cap-slide-to-delete
---
Yesterday [Jeff Atwood wrote](http://www.codinghorror.com/blog/2010/03/the-opposite-of-fitts-law.html) about [Fitts’ law](http://en.wikipedia.org/wiki/Fitts'_law) and the contrapositive in regards to _big, bad eject buttons_. The point of his piece was that you shouldn’t have buttons with irreversibledestructiveresults be right next to buttons with more frequent use.

I was actually more intrigued by a couple of the commentors when they mentioned a real life example of excellent “eject” button UI: these incredibly important buttons have a safety cap over them.

> Put a safety cap over the button. You know, like in the movies :-) You cannot press the button, because there is a “cap” on top of it. You first have to “open the cap” before you can press the button below it. In case of software, the button would be not directly visible, you first have to press somewhere to make the button appear and then you can press it
> 
> <cite>—<a href="http://www.codinghorror.com/blog/2010/03/the-opposite-of-fitts-law.html#comments">Mecki</a></cite>

[<figure class="tmblr-full" data-orig-height="500" data-orig-width="375"><img src="https://64.media.tumblr.com/85a3888e01b0080c0492f1e65284b4f6/5ff4516258856f17-20/s540x810/625b927ee65da227dd818ce5c7a013f6ba5be53f.jpg" alt="Emergency Button covered by Safety Cap" data-orig-height="500" data-orig-width="375"></figure>](http://www.flickr.com/photos/daquellamanera/317329370/)

Yea, that. **There’s no way you can accidentally push that button**. (Wouldn’t it be nice if that were true?)

I’ve seen this idea before, I’m sure, but wanted to throw it together anyways. When you click the delete button, or any other important, damaging button, the delete button disappears and a slider appears, requiring you to move the slider over to complete the delete action. This only took a few minutes, so I wouldn’t necessary use this code for production. Regardless, [here’s a proof of concept](http://mootools.net/shell/XG8qG/2/): (if you’re in a reader, you should come through to the site to see this).

<iframe style="width: 100%; height: 300px" src="http://mootools.net/shell/XG8qG/2/embedded/"></iframe>
