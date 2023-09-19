---
layout: post
title: Rich Text Editor in Mootools
date: '2009-03-13T16:19:00-04:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/706995594/rich-text-editor-in-mootools
---
With the release of Firefox 3, we find all major browsers now support a single interface (mostly) for allowing the Browser to do all of our Rich Text editting needs. This allows us to set a single property, contentEditable, and then that element accepts rich text processing. No more loading in an iframe, having document.domain issues, and having little control over the style of the iframe. This, however, does mean Firefox 2 and below are excluded from the party.

#### Midas API

In Firefox 3, they call it [Midas](https://developer.mozilla.org/En/Midas) , and it’s a series of simple commands that mimic what Internet Explorer has [had for several versions](http://msdn.microsoft.com/en-us/library/ms533049(VS.85).aspx) . And it’s all called through a simple command:

document.execCommand(‘bold’,null,null);

The above would make anything selected in an editable element become bold, or if no selection, make the next things you type be in bold. Just like you’d expect in Word or some such. So the amount of Javascript that has to happen is minimal, allowing the browser to do most of the work (which is faster than Javascript interpreting and processing).

<figure class="tmblr-full" data-orig-height="133" data-orig-width="440"><img src="https://64.media.tumblr.com/8b49d0000da069af3a4fdfc5d437e902/1993cab508322e7e-49/s540x810/b7d00846f1a5fee0a50f951598a2a94ee9b3d11e.png" data-orig-height="133" data-orig-width="440"></figure>

What’s most exciting to me, is that since so little Javascript is needed to do most every command, _the project isn’t huge and unwieldy_ . It allows for more of the project to be aimed at a killer interface. (Mine isn’t.) It also makes it very easy to edit, and add new buttons, since the use of a button is mostly centered around the document.execCommand function.

What’s nice about the ease, is that we were able to use a similar solution in Prototype for [our content editor](http://blazonco.com) , and it’s very easy to add a button that allows picture selection, instead of typing in a picture url.

#### Download

Demo and code is hosted on [github](http://github.com/seanmonstar/moo-rte).

#### Volunteers?

This is an early build, prompted by [Chris](http://og5.net/christoph/), and isn’t perfectly polished. But it does get the job down. It could do with a nicer looking toolbar. And I’d like to get the Range and Selection API’s working, so that you don’t have to use the boring prompt box to insert links. If you try to use a modal prompt, since its the in the same window, the selection will change when the user types in the link, and will no longer insert the link around the (now un-)selected text.

Anyone can use it as is. And if you have suggestions, leave them in the comments and I’ll try to work them in.

