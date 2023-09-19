---
layout: post
title: Protected Methods in MooTools Classes
date: '2009-09-04T11:00:00-04:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/708692497/protected-methods-in-mootools-classes
---
In Javascript, elements don’t have a native way to hide properties. So we have to come up with creative ways like using [closures](http://snook.ca/archives/javascript/no-love-for-module-pattern/). In MooTools 1.2.3, we have a way to protect class methods.

#### Private vs Protected

The difference between private and protected variables in most languages is that private can **only** be used in the same class. Protected is similar, but it can also be used from a class that extends the base class. Private properties are off limits to children.

### Engines must Start before Running

When writing your classes, you can specify methods that should be protected by building the function in a similar way you bind functions:

    var Engine = new Class({
        initialize: function() {
    
        },
        start: function() {
            //... starting magic ...
            this.rotate();
        },
        rotate: function() {
            var rotate = true;
        }.protect();
    });

See that `.protect()`on there? That function is now protected. Which means, that only other internal methods can call that function. If we instantiate a new Engine, we can’t tell it to run ourselves. We can only tell it to start, and the engine will do all its starting magic before making it run. Capiche?

Now, when you’re designing your classes, keep in mind the API you want to present the class’ users. Several MooTools classes only share a couple methods for public use, and many other functions are written for internal use only. With protect, you can make sure none of those methods are relied upon, or possibly called before another method that is supposed to be called first.

