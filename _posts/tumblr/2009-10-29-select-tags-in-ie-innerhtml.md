---
layout: post
title: 'Select Tags in IE: innerHTML'
date: '2009-10-29T12:08:00-04:00'
tags:
- javascript
- bug
tumblr_url: https://seanmonstar.com/post/708829330/select-tags-in-ie-innerhtml
---
I just wanted to document this rather frustrating bug here, so I can look it up later, and hopefully help anyone else who is running into something similar. This bug involves `select` tags, specifically setting their `innerHTML`property.

I had a list of options to give to the user, and a select box would work perfectly. Since DOM methods have [burned me in the past](http://seanmonstar.com/blog/dont-use-the-dom-to-insert-flash/), I felt `innerHTML` was the safer route. It appears, that they both have safety curves like a sine and cosine graph.

<figure class="tmblr-full" data-orig-height="222" data-orig-width="322"><img src="https://64.media.tumblr.com/45a7f3c1268e5345b73b1f373f391dc1/a18c2b652749ba34-73/s540x810/9baf9c961b135c0b67c06d77f6edf7b64aefa704.jpg" data-orig-height="222" data-orig-width="322"></figure>

#### The Buggery

Here’s what I thought would work of stupendously!

    var options = '<option>' + item.options.join('</option><option>') + '</option>';selectEl.innerHTML = options;

It works in Firefox, Safari, Chrome, etc. _Except Internet Explorer_. I first noticed in IE7, and so when I went to debug (using [IE8’s sweeter debugger](http://twitter.com/seanmonstar/status/3540098404)), I noticed it even breaks in IE8.

It disregards completely the first `<option>` in the string. Go ahead and try it yourself. Open IE8, press F12, go to Script, and try this super simple task:

    var selectEl = document.createElement('select');
    selectEl.innerHTML = '<option>this one breaks</option><option>we dont work because the first one is broken</option>';
    document.body.appendChild(selectEl);

#### Solution

It breaks even that. So I resorted to DOM methods to build my select field.

    for(var i = 0; i < item.options.length; i++) {    
    	var option = document.createElement('option');    
    	option.innerHTML = item.options[i];    
    	select.appendChild(option); 
    }

