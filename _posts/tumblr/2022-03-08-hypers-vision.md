---
layout: post
title: hyper’s Vision
date: '2022-03-08T12:56:52-05:00'
tags:
- rust
- rust-lang
- http
- hyper
tumblr_url: https://seanmonstar.com/post/678179333918097408/hypers-vision
---
This year is the year we release [hyper](https://hyper.rs) 1.0. As pointed out in the [timeline](https://seanmonstar.com/blog/hyper-10-timeline/), the first thing we need define is the [vision](https://github.com/hyperium/hyper/pull/2772).

The vision is a high-level, abstract view of what hyper _is_. It’s the destination. It defines “where we are going”, no matter where we are along the journey, or what version we get to along the way.

### Why define a vision?

hyper has been around for [a little under 8 years](https://github.com/hyperium/hyper/commit/886551681629de812a87555bb4ecd41515e4dee6). We’ve done a lot of experimenting, learning, trying this and trying that. It has always had an _implicit_ vision, but it’s important to write it out. This makes the vision explicit, clear to everyone. It’s no longer just whatever Sean keeps in his head.

Defining a vision is a way to keep us honest. It provides a measuring stick to determine if we’re getting closer or farther away. It defines a framework on how to make decisions that affect hyper. It empowers others to make those decisions. It allows the project to survive as maintainers[^1] and collaborators come and go.

### A Charter and Tenets

The [vision](https://github.com/hyperium/hyper/pull/2772) begins with a charter and tenets.

A charter is the _what_. What is it in one sentence? What is hyper?

> hyper is a protective and efficient HTTP library for all.

The tenets are _how we do that_. How we make the charter a reality. How we make decisions so that hyper can accomplish its charter. We do so when “hyper is …”:

1. Open
2. Correct
3. Fast
4. HTTP/\*
5. Flexible
6. Understandable

As much as possible, hyper should strive to fulfill all of those feelings.

Some feel that tenets or principles must be unique. Others note that clearly you would never want the opposite. Yes, that’s obvious. No one wants a buggy, or slow software. Defining tenets has another purpose, though.

Defining tenets sets the _priority_ amongst the principles. If you have to pick between two of them, how do you decide which wins? In this case, we’ll work _very_ hard to make things both correct and fast, but if there’s a conflict, we will choose a slower correct option over a faster incorrect option. The vision goes into much more detail about each tenet, and includes real specific examples of how hyper _already_ has made decisions using them.

You may notice some minor mixing of human and engineering principles, such as “protective” and “open”. The mixture is purposeful. We remember that we make software for humans. What is engineering without the humans? It’s too easy to forget that. hyper’s vision makes sure that we consider humans when making engineering decisions. Still, there is room to expand how exactly the humans leading the project [should act](https://github.com/hyperium/hyper/blob/master/docs/CODE_OF_CONDUCT.md) with a collaborator guide in the future.

### Use cases of the Library

The next part of the vision includes hyper’s use cases. We collected these at the same time as defining the the tenets, to help make sure that hyper’s users actually cared about the same thing hyper does.

This was extremely helpful in realizing the right balance, and recognizing that being flexible was higher priority than being easy for newcomers. There has been a balance in hyper of trying to provide a low-enough level interface, and also providing an easy and powerful higher-level API. Trying to provide both at the same time meant both suffered. By accepting that the higher priority need is to be flexible, we can remove some of the restraints that were imposed because of the higher-level API, and be even more flexible.

That doesn’t mean we purposefully make it harder, though. It should still be understandable for newcomers. One of the first things we do for them is to recommend they look at libraries built on top of hyper that are designed to be easy-first. We’ll move some of the higher-level building pieces into a `hyper-util` crate.

### And More

There’s more to hyper’s vision that I don’t outline here. [The vision is currently a proposal](https://github.com/hyperium/hyper/pull/2772), and accepting feedback. Even after we merge, it’s not set in stone. The vision can adapt as we learn new things in the future. But this is the [first step in shipping hyper 1.0](https://seanmonstar.com/blog/hyper-10-timeline/). We can’t get anywhere if we don’t know where we are going. Now we do!

After this, we need to map out _how we get there_. That’s the roadmap. That’s the next piece that’s coming.

Come [join us](https://github.com/hyperium/hyper/pull/2772), let us know what you think, and help us make the protective and efficient HTTP library **for all**!



[^1]: Even me!

