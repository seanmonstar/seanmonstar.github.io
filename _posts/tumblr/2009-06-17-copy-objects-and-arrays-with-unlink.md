---
layout: post
title: Copy Objects and Arrays with $unlink
date: '2009-06-17T13:00:00-04:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/708644598/copy-objects-and-arrays-with-unlink
---
Basically, if you don’t know what happens when you have multiple variables pointing to the same Object or Array, then check out this [quick and dirty article](http://james.padolsey.com/javascript/deep-copying-of-objects-and-arrays/) and read the first half or so. Or if you don’t want to bother, I’ll give you a snippet to paste into Firebug.

    var a = ['one','b',3,false];
    var b = a;
    a[1] = 'BEEEE';
    console.log(b[1]);

#### Reference, pointer, whatever

Caught on to what was happening there? We made one array, then assigned that array to another variable. That variable simply points at the same array, it’s not copied. Now, when we change the array that’s assigned a, we find the results of the change appearing b also.

This is because Javascript assigns complex types (notably Objects and Arrays (Functions and Strings count too, but they’re immutable, so it doesn’t matter)) by pointers. Usually called pass by reference, there might be some argument over the proper verbage. Ultimately, it doesn’t matter much what you call it when you talk to yourself. It just matters that you understand what happens.

#### Making Distinct Copies

Now, sometimes, you may want a copy of the array or object instead of a reference, so if the original gets modified, your copy stays out of that sometimes confusing mess. There are a couply ways of copying an array or object, but what matters here, is that if one of the values is itself an array or object, you’ll need to make a copy of that as well. Otherwise you’ll get pointers in your facade of a copied object.

##### $unlink

The MooTools Core (1.2.2 as of writing) provides a function by the name of $unlink that solves this issue for us. Provide an array or object as an argument of $unlink, and it will return a copy that has been checked to make sure it has no back references. Let’s do so to our original example.

    var a = ['one','b',3,false];
    var b = $unlink(a);
    a[1] = 'BEEEE';
    console.log(b[1]);

See? They’re different. Go ahead and play with making arrays and objects that contain nested objects, and see what happens. Then you’ll see the power that `$unlink` gives you.

