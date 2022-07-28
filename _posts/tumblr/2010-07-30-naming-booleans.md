---
layout: post
title: How I Name Boolean Variables
date: '2010-07-30T09:04:50-07:00'
tags:
- opinion
tumblr_url: https://seanmonstar.com/post/880164961/naming-booleans
---
Naming variables is important stuff. Besides the classic “classes should be nouns, methods should be verbs” stuff, I find another very common naming convention I use is in naming my Booleans.

Particulary, I like to name them so they are a question. You can read the name of the variable, and feel you were asking a question about some object. And the question **must** be a yes/no question for it to work. No `howAreYou` variables. I make sure that it truly does sound like a question. I don’t like using single word adjectives or adverbs, like `running` or `recommends`. I’ll add `is` or `has` to make it sound like a question.

Some examples:

    server.isRunning
    book.isRecommended
    dog.canBark
    table.shouldReplace

It’s much easier to tell by looking at them that they are boolean values. Whereas, something like `item.likes` could actually be a number, of how many people liked the item. `item.isLiked` tells me that something liked the item. **It’s going to be a yes or no answer to my boolean question.**

