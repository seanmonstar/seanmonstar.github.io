---
layout: post
title: Why Object.prototype is not the place for Hash methods
date: '2010-11-12T09:00:00-08:00'
tags:
- javascript
- mootools
- planet
tumblr_url: https://seanmonstar.com/post/1554012583/object-prototype
---
You might have heard that extending the Object.protoype is [verboten](http://erik.eae.net/archives/2005/06/06/22.13.54/). But you might realize that one of the most famous issues with extending Object can be solved with the handy `obj.hasOwnProperty(key)` line. Besides, developers who like picking MooTools likely do so (at least partly) because it’s much more elegant to call methods on objects than pass objects into functions. I’ve been asked why MooTools choses to keep a separate Hash class instead of putting those methods directly on the Object prototype. Here’s my reasoning for why.

### Object is an Easy Hash

In JavaScript, the object literal notation makes it very easy to create hash-like structures. However, it does not provide that hash with methods to access keys, or do a forEach loop over, do a map function, and the like. Some developers might reach for the Object’s prototype to add in these useful methods. However, you might remember that **all** objects inherit from Object. Which essentially means everything (Array, Function, user-defined class).

### Dog isn’t a Hash

Now imagine, defining a class like Dog. Or how about User, since that’s much more real-world. Does it make sense for User to inherit Hash methods? Would you in any other language write `class User extends Hash`? Can you imagine being able to call `someUser.map(fn)` or `someUser.keys()`? It’s silly!

You might be thinking that it doesn’t matter, you would never need to make those method calls, so they’re practically invisible. But they’re not. The point of decent code is modularize information and logic, not mix in pieces we never need.

It’s especially easy to just include the Hash class from MooTools More if you need hash capabilities. It’s 6 extra characters around your object literal, and then you don’t pollute any other class. Hurray!

