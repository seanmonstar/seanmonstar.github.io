---
layout: post
title: 3 Tips When Switching to Python
date: '2010-01-14T12:30:00-08:00'
tags:
- python
tumblr_url: https://seanmonstar.com/post/708913184/3-tips-when-switching-to-python
---
If you write a lot of Javascript or PHP, there are a couple of habits you might be used to that need to change a bit when you switch over to Python.

1. Accessing a property in a dictionary with a variable
2. Setting properties on objects with a variable
3. Using While with a function call

#### Check in first

When looping through a list or dictionary, it’s not uncommon to compare the current indexed value to a value in a different list or dictionary. However, doing that will quickly teach you that trying to access a key that doesn’t exist will raise a `KeyError`.

After my first `KeyError`, I first tried to wrap the comparison in a `try` block. But that can start to look unwieldly:

    try:
    	val = params[key]
    except KeyError:
    	val = None
    if val:
    	#...

Alternatively, Python dictionaries have a `get` method. Hence we do this, and it’s all pretty!

    val = params.get(key, 'defaultVal')

#### You can just setattr

A while ago, when doing some [initial Django development](http://seanmonstar.com/2022/07/28/2009-11-25-extending-django-models-managers-and-querysets.html), I tried to naively handle submission of forms the same way I do in PHP. I loop through each key value pair in the POST dictionary, and assign it to an instance of the model I want to insert. No worries if extra information has been submitting, the model will send data that we have specifically set at fields in the `class` definition. However, objects in Python don’t allow item assignment like PHP or Javascript. Every object does have a personal ` __dict__ ` that I could access, but then I can get `KeyErrors` as the above example shows.

[Denis Otkidach showed me](http://stackoverflow.com/questions/1614804/copying-values-from-a-dictionary-into-an-object-in-python) Python’s [setattr function](http://docs.python.org/library/functions.html#setattr), which lets me do exactly what I wanted.

    for key, value in POST.iteritems():    
    	setattr(my_model, key, value)

#### While True, loop foreverrrrr

Often times, when you don’t have a predetermined length of something, you’ll use `while` to do your looping. A common occurrence of this is when reading in a file. You call a function, and store its return value, and as long as that value is usable, do your loopity loop stuff. However, in Python you can’t do assignment inside a condition for a control structure like `while`, probably because Guido likes to prevent bad practices from being possible in his language, and that is usually a bad practice unless you know what you’re doing.

There maybe a more elegant way of handling this, but i resorted to making an infinite loop that breaks on a bad condition:

    while True:
    	val = func()
    	if val:
    		pass
    	else:
    		break

If you write a lot of Javascript or PHP, there are a couple of habits you might be used to that need to change a bit when you switch over to Python.

1. Accessing a property in a dictionary with a variable
2. Setting properties on objects with a variable
3. Using While with a function call
