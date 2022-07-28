---
layout: post
title: Making Objects in an Array Unique
date: '2009-02-06T06:01:00-08:00'
tags:
- php
tumblr_url: https://seanmonstar.com/post/707138900/making-objects-in-an-array-unique
---
I was doing some sorting of Models in PHP. Unfortunately, I didn’t have the luxury of letting the SQL do it all for me. Usually it does. But besides sorting, I had to make sure I didn’t have any duplicate entries, since I was merging arrays with different queries. My first hope was PHP’s array\_unique method.

array\_unique($array) will return an array with all duplicate values removed. What’s deemed duplicate is when the _string values are the same_ . This doesn’t work for objects. I had used usort previously, soI looked for something similar like a uunique or something. No such luck.

I did find array\_filter, which lets me specify a function the will determine what values get kept in the array and what gets ditched.So I created a simple dummy function, inside make a static array, grab the id from each model value in the array. Iadd the id to the static array, and then check the next value to see if theid has already been stored.

    function unique\_obj($obj) { static $idList = array(); if(in\_array($obj-\>id,$idList)) { return false; } $idList []= $obj-\>id; return true; } $posts = array\_filter($posts,'unique\_obj');

