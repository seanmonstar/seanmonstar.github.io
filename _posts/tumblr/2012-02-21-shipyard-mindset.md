---
layout: post
title: The Shipyard Mindset
date: '2012-02-21T16:12:08-05:00'
tags:
- bestof
- javascript
- programming
- shipyard
- planet
- mozilla
- webdev
tumblr_url: https://seanmonstar.com/post/18026837126/shipyard-mindset
---
I’ve been working quite a bit on this little [JavaScript MVC framework called Shipyard](http://seanmonstar.com/blog/what-is-shipyard/). Those who know me may recall that I used to write a lot about MooTools, and may wonder why I’ve moved off it and am writing my own framework instead. I figured I’d take the time to explain why I felt the need existed for Shipyard, and the goals it tries to accomplish:

- Being truly modular
- Command line tests run after each save
- Easy access to data from various sources
- Auto-updating views

Let’s get to it.

### Modularity

JavaScript has a tendency to turn into spaghetti pretty quickly, what with the callback parties and people saying it’s “just a scripting language”, applications tend to lack structure. MooTools has had a modular design from the beginning, by separating pieces of functionality into individual Classes. This concept has carried over into Shipyard, but Shipyard does so much more.

With other frameworks, such as MooTools or YUI, developers are asked to pick the components that they want to use when downloading the library. The goal is that people only ever download the JavaScript they need for their application. Unfortunately, people usually just download the full “default” build, that contains everything, because they don’t know what they want yet. It’s certainly a pain to try to be smart about your selections at first, and then later realize you need to download a couple more modules (plus all their dependencies) mid-development. So most people download the entire thing, and never chop out the unneeded afterwards.

Shipyard says that you should download the entire thing from the start. Download the entire repo, all the modules. As you write your application, you specify your dependencies naturally in each module (using `require`), and so you never need to look at a list and pick what you _think_ you might one day sort of use. It’s all there on your computer, and you just use it naturally. When it comes time to ship to production, the build step (you’re already minimizing anways, right?) that comes with Shipyard only bundles the dependencies you specifically used. **Shipyard takes an active stance in reducing the amount of wasted bytes that your users will have to download.**

### Testing

Testing is great. It’s the law. It’d be a good idea. Something like that, right? In many other frameworks, it’s very easy to write test suites for your applications, but JavaScript applications are strikingly absent of tests. Part of the reason is that many test frameworks make it difficult to test. I need to be able to test with the simplest of commands. Even have the possibility to test on a pre-commit hook, or even _per save_. That means testing needs to be easy, and fast. If it’s not, it just won’t happen.

**Automatic tests run after each save in JavaScript, you say?** But it’s hard to test JavaScript from a command line, because you need to test in browsers, right? Well, yes, you should do that too, but so much of our JavaScript applications nowadays is code that has nothing to do with the browser. So much of it can be isolated and tested in units. And while you should browser test your application as well, if a test breaks on the command-line, you know it will break in the browser before even having to load the test page. Fail faster.

Shipyard helps do this by makng it’s test runner run with nodejs. With the [strict usage of the `dom` module](https://github.com/seanmonstar/Shipyard/blob/master/doc/topics/dom.md) whenever touching anything global and DOM related, the test runner is able to make the `dom` module work in nodejs with the help of jsdom. So you can actually test expected DOM behavior from the command line, each time you save your file.

You could even put your JavaScript test suite on CI, similar to how Shipyard’s own test suite runs on [travis-ci](http://travis-ci.org/#!/seanmonstar/Shipyard) with every commit.

### Model Syncs

Getting into the more MVC part of Shipyard, I had explored this [idea of various sync locations with Models before](http://seanmonstar.com/blog/mvc-in-mootools-models/). The idea is that applications have data, and we structure it with Models. The data comes from somewhere, and while it used to only ever come from the host server, increasingly it is coming from various sources. A common example that would benefit from this is an application with offline mode. You need the data of your models to sync with the server, but if the user is offline, you want the data to save locally, perhaps in `localStorage` or `IndexedDB`, and then be able to send the data to the server at a later point. Perhaps you want to cache the data in `localStorage`, and so when the user comes back to your site, you first look there, and then fall back to asking the server for the data.

It should be as simple as:

    Recipe.find().addListener('complete', function(recipes) {
        if (recipes.length === 0) {
            Recipe.find({ using: 'server' }).addListener('complete', listRecipes);
        } else {
            listRecipes(recipes);
        }
    });

Shipyard makes it so.

### Automatic Views

The DOM sucks. It’s powerful, but it’s complicated, and has inconcistencies. I don’t think developers should have to touch the DOM, in most cases. Instead, Shipyard exposes Views. They’re kind of like Elements, but far more powerful. Specifically, Views can be made up of several elements, and not bother you about it. As well, you don’t have to fret over which of those elements needs event handlers, you just listen to events from the View itself.

Even cooler is the idea of binding data to Views. My first foray in JavaScript MVC had me re-rendering entire sections of the DOM when things had changed. Not only is that weak, performance-wise, but it’s boiler-plate that I had to worry about. You can end up with several places in your UI that reflect the same data, shown in slightly different ways, and then you’re left with remembering each place when you offer a new way to alter the data. _Of course_ the UI should update when the data changes, I just shouldn’t have to remember that myself. I am only human, after all. Other frameworks have this (I met it when using Adobe’s Flex Builder), and so does Shipyard.

### Choo-choo

If you were nodding when reading the above, then [get on the Shipyard train](https://github.com/seanmonstar/Shipyard). Development continues strongly, it already powers a [big application](https://builder.addons.mozilla.org), and I hope it works out for you.

