---
layout: post
title: Information Obsession
date: '2013-03-20T19:41:00-04:00'
tags:
- rss
- google reader
- feedmonstar
- information obsession
- planet
tumblr_url: https://seanmonstar.com/post/45871523111/information-obsession
---
_I’m a week late to the “[Google Reader dies](http://googlereader.blogspot.com/2013/03/powering-down-google-reader.html)” news, but I wanted to collect my thoughts and read other people’s knee-jerk reactions first._

I am quite interested in human data consumption. [Obsessed](http://seanmonstar.com/blog/prioritizing-my-tiny-inbox/), even. Like, I constantly re-analyze my current setup, making sure I’m getting all the information I want, while keep as much junk out as possible. Maybe sometimes also re-analyzing my obsession with information obsession.

### My Current Digs

1. I subscribe to high signal-to-noise blogs where I want to read just about every article using **RSS**. These are typically opinion blogs where I enjoy reading what the author has to say. Pretty much nothing in here talks about the latest news. Of all the available choice, I’ve always used Google Reader, since… eh, who’s kidding who, there was no other proper choice. It had the added benefit of being syncable with Flipboard, Feedly, and any other mobile feed reader I tried.

2. I get my latest news, and otherwise follow “blogs” or personas that post interesting links via **Twitter**. This seems like the least-worst way to battle all the noise, since I only have to see 140 characters to decide if it’s interesting to me. Plus, if it’s not, I don’t need to bother with “Mark All as Read” bah-log-na.

3. That’s it.

With Google Reader dying, I need a new place to move my “must reads” RSS subscriptions. Some of the Internet seems to think we shouldn’t need that anymore, but I’ve always seen my consumption as being from 2 buckets: **high-signal must-reads** , and **abysmally low-signal may-be-interesting-but-oh-god-I’m-sinking snacks**.

I’ve put a lot of thought into this obsession, and have 2 key objectives when it comes my reading: try to never miss anything I’d truly enjoy, and ruthlessly remove anything that I _won’t_ enjoy.

To that end, a couple years ago, I started working on my own idea of a feed reader, that would try to solve these issues, and not just be a simple interface on “dumb pipes”.

### feedmonstar

![feedmonstar screenshot](https://lh4.googleusercontent.com/-rOZZHV3CSPk/TkQWjE8L0yI/AAAAAAAAANs/9daDF17aWVs/s910/feedmonstar.jpg)

You’ve likely heard of [Fever](http://feedafever.com). It has a cool concept of grouping sites together when they share links to a “hot” topic. I found this to be genius, but then was quickly let down when I wished it would do more. I didn’t want a separate list of what’s currently “hot”. I simply wanted to read my list at the end of the day, letting a tool group up articles that were talking about the same thing. That way, I could read them all at once, or _skip_ them all at once, and have them removed from my river of news.

Additionally, I wanted something like Google Reader’s “Sort by magic”, only that was magical about **you** , not the greater hive mind. So many times, it would tell me that an article on Smashing Magazine about “Top jQuery Plugins” was the most important thing to read. Clearly, it knew nothing about me.

So that was the goal of feedmonstar, and the prototype really started to do just that in an alpha state, before I got busy at Mozilla, plus my server was having issues as my “magic” sorting required a lot of memory to dynamically sort, score, and group the feed on each request.

If that sort of thing interests you, [get in contact with me](http://seanmonstar.com/about). I still have all the code, and could be bothered to set it up on another host to try and work on, if there’s interest.

