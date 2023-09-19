---
layout: post
title: ParentNode Misbehavior in IE
date: '2008-11-19T15:51:00-05:00'
tags:
- javascript
- bug
tumblr_url: https://seanmonstar.com/post/707125202/parentnode-misbehavior-in-ie
---
I wrote a simple script to pop-up a modal-like element when clicking on an “Add” button on the page. I wanted to insert the element into the DOM on first click, and on subsequent clicks, simply reset the form. A bug popped up in IE7 (surprise) that prevented me from checking if I had inserted the element already.

#### What I tried to do

I performed the case check by simply checking the parentNode of the element, which obviously shouldn’t exist until it’s inserted into the DOM.

    if(!this.form.parentNode) $('Admin').insert(this.form); else this.form.reset();

As I expected, worked perfectly. _Until I popped open IE7_.

##### IE Ignores Specs

Apparently, IE7 assigns a parentNode to nodes upon creation, before it has been inserted into the DOM. Well that’s silly, because the specifications say nothing about setting the parentNode. In fact, they state the opposite:

> However, if a node has just been created and not yet added to the tree, or if it has been removed from the tree, this is `null` .

That’s fine. Doing some debugging, I find there is indeed a parentNode. And that parentNode has a nodeType of 9, or DOCUMENT\_NODE. So it makes a new, off-screen document to hold onto created Elements. Whatever.

##### Excusing IE

A simple modification to my if statement fixed everything, once I finally knew IE gives it a parentNode. The good part (_there’s good here?_ ), is that it assigns the new Element a parent that only the HTML node should have, a document. So here it is modified:

    if(!this.form.parentNode || this.form.parentNode.nodeType == 9) // 9 == DOCUMENT\_NODE, but IE is freaking retarded. I'm ganna kill Microsoft 100 times over. $('Admin').insert(this.form); else this.form.reset();

