---
layout: post
title: Making a Toggle All Checkbox
date: '2008-09-16T09:00:00-07:00'
tags:
- javascript
tumblr_url: https://seanmonstar.com/post/707004893/making-a-toggle-all-checkbox
---
<figure class="tmblr-full" data-orig-height="150" data-orig-width="400"><img src="https://64.media.tumblr.com/295b17d2f19e9f3babd4aed4c7be1ec0/e39ec8c2b5f78e01-a7/s540x810/a721a58645137e8467c2a4f155b3c14dfe40a259.jpg" data-orig-height="150" data-orig-width="400"></figure>

You’ve seen them on other sites. When you have a long list of checkboxes, and you know you need to check them all. Thankfully, the developer gave you a “Toggle All” and “Check All” box at the top, so you don’t have to check each individual box. They don’t make or break an application, but they _subconsciously make a better user experience_. It’s really easy to get this working, and I can show you how to do it in Mootools, Prototype, and even without any framework, since if this is the only thing you need in Javascript, you need the overhead of a framework to do this.

I recently had to make a permissions page for a CMS, allowing users to set the permissions of new users for various features. Adding a way to toggle all options would be nice, because there’s a lot of options! Here it is with _Mootools goodness_, thought for the CMS I wrote it with Prototype:

#### Mootools

    $('toggler').addEvent('click',function (e) { var toggle = $('toggler').checked; $$('#formId input[type=checkbox]').each(function(check) { check.checked = toggle; }); });

I’ve added an event listener to the checkbox with id “toggler”, listening for a `click`. When clicked, I store its `checked` state (true if checked, false if unchecked), and then set every other checkbox to the same state. This way of doing it, as opposed to just using `check.checked = !check.checked` (using NOT to pick the opposite state), prevents double toggling the Toggler checkbox. Here it is originally in _Prototype:_

#### Prototype

    $('toggler').observe('click',function (e) { var toggle = $('toggler').checked; $$('#formId input[type=checkbox]').each(function(check) { check.checked = toggle; }); });

The only difference between Mootools and Prototype is the `attachEventListener` implementation: _Mootools uses `addEvent`, Prototype uses `observe`_.

And like I said in the beginning, if this is your only need of Javascript (unlikely), I recommend not bogging down the page with a framework, and _use native Javascript_. It’s really not hard. Here it is:

#### Native

    var toggleFunction = function() { var toggle = document.getElementById('toggler').checked; var inputs = document.getElementsByTagName('input'); for(var i = 0; i \< inputs.length \< i++) { if(inputs[i].type == 'checkbox') { inputs[i].checked = toggle; } } } //If Good Browser if(window.addEventListener) { document.getElementById('toggler').addEventListener('click',toggleFunction(),false); // If IE } else if (window.attachEvent) { document.getElementById('toggler').attachEvent('onclick',toggleFunction()); }

If you’re used to only using frameworks, this looks like more work. And it is, slightly. But if you’ve done any kind of programming, you realize _it’s just a simple `for` loop_, used normally.

Of course, this does show-off some of **the things frameworks do for you**. The framework will check the type of input for me, instead of me having to write an `if` statement. It also gives you a cross-browser `addEvent` function, instead of having to check for _browser inconsistancies_.

