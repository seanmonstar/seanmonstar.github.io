---
layout: post
title: Closures Break my For's
date: '2009-12-10T12:30:00-05:00'
tags:
- javascript
tumblr_url: https://seanmonstar.com/post/708874716/closures-break-my-for-s
---
 **I love closures**. They are an excellent tool any Javascript programmer should have in his tool set. They let you do fantastic things, and are the way things like the [Module Pattern](http://yuiblog.com/blog/2007/06/12/module-pattern/) are possible. But they can also be tricky. I’ll show you a couple ways they’ve managed to fool me, so that you can be aware of them when you use them in your programs.

### When Doing a Simple `for` Loop

`for` loops are nothing special to programmers. And they’re nothing special in Javascript. So why would you need a closure in a simple `for` loop? Whenever you’re binding functions to things inside that `for` loop, you likely need a closure.

    function Highlighter(ul) {
        var that = this;
        var subjects = ul.getElementsByTagName('li');
        for(var i = 0; i < subjects.length; i++) {
            addEvent(subjects[i], 'click', function() {
                that.highlight(i);
            });
        }
    }

A seemingly harmless `for` loop. We’re just looping through all the list items in a list, and [binding click events](http://seanmonstar.com/blog/cross-browser-addevent-without-frameworks/) to them, so they will highlight when clicked. If you were to create a new `Highlighter` like this, and then click on a list item, you’d get an error. Inspecting the error you would see that, hey! `i` is equal to the `length`. _What the heck!_

Well, when you created all those new functions in the `for` loop, to bind them, you gave them the variable `i`. But `i` wasn’t part of the functions local scope, so it creates a closure. `i` can change. And it does! The `for` loop eventually makes it be equal to the length, which fails the expression check in the `for` loop. In most programming languages, we’d think that’s it for `i`. But no, `i` lives on as long as those functions exist and care about i. ([Read up on closures](https://developer.mozilla.org/en/Core_JavaScript_1.5_Guide/Working_with_Closures))

So we can use a closure to make sure every function we create gets it’s own, paused variable to use, so it won’t get changed with the `for` loop.

    function Highlighter(ul) {
        var that = this;
        var subjects = this.subjects = ul.getElementsByTagName('li');
        for(var i = 0; i < subjects.length; i++) {
            (function(index) {
                addEvent(subjects[index], 'click', function() {
                    that.highlight(index);
                });
            })(i);
        }
    }

Now, every pass of `i`, we create and execute a function that accepts an argument that we call index. Since it executes immediately, it passed the value of `i` at that moment.

### Let’s take it one step further

In my `for` loop, I’m also relying on another variable outside my bound event function: `subjects`. Theoretically, I could write something later that would affect `subjects`. Or someone else could. Perhaps if I wrote some more in my constructor function, and altered the array. More likely, since that is a `NodeList`, it can change if the DOM ever changes, such as adding more list items, or removing them. If that happens, then we’ll have issues besides just our variable being different.

Nonetheless, we could make our closure less reliant on that and other variables, by passing them as arguments.

    function Highlighter(ul) {
        var that = this;
        var subjects = this.subjects = ul.getElementsByTagName('li');
        for(var i = 0; i < subjects.length; i++) {
            (function(list_item, index) {
                addEvent(list_item, 'click', function() {
                    that.highlight(index);
                });
            })(subjects[i], i);
        }
    }

We no longer access the variable from outside the function. We’re passing a value at the function executing. Again, if later that array changes, our function is still safe (disregarding `that.highlight(index)` will likely have the wrong index).

### Frameworks Help Remove This Problem

This issue comes up commonly in `for` loops. That is why in frameworks, using a `forEach` method tends to protect us from all of these. `forEach` lets us create a function that will already have the value from the array passed as an argument, so we don’t have to worry about the index value changing.

#### In MooTools

    function Highlighter(ul) {
        var that = this;
        var subjects = this.subjects = ul.getElements('li')
        subjects.each(function(el, index) {
            el.addEvent('click', function() {
                that.highlight(index);
            });
        });
    }

Most other frameworks have a `forEach` method, as well as [modern Javascript itself](https://developer.mozilla.org/En/Core_JavaScript_1.5_Reference/Objects/Array/ForEach). However, if you’re not working with a framework, it’s good to know why your closures are breaking.

