---
layout: post
title: You Don't Always Need Identity Operators
date: '2009-09-10T08:00:00-07:00'
tags:
- opinion
- javascript
- php
tumblr_url: https://seanmonstar.com/post/708732601/you-dont-always-need-identity-operators
---
In two languages that I use often (PHP and Javascript), there’s 2 different equality operators when comparing values. It’s become quite common to see places expressly tell you that you should only ever use one of them. That the other is evil. People see this, and then point fingers whenever you use 2 equal signs instead of 3. Here’s perfectly valid reasons to use equal operator (==) instead of identity(===).

In Javascript, depending on the browser, getting the value from inputs, especially when I expect to get a number value, can often times be a number in string format. And in those situations, _I don’t care_ . If I’m looking for someone to be 21, I don’t care if they claim to be 21, or ‘21’. They both work for me.

My options are to use a nice, simple equal operator, or do something **more verbose** , **harder to read** , and **suckier to write** , to make sure I use the identity operator.

I’ll do it this way:

    if(elem.value == 21) {
    	//enter bar.
    }

So I don’t have to write this sillyness:

    if(elem.value === 21 || elem.value === '21') { 
    }
    //or
    if(parseInt(elem.value, 10) === 21) {
    }

Alternatively, I could write some method in PHP that manipulates a string of input. Before I do anything to it, I’m going to verify it’s something I can manipulate. If it’s null, or false, or an empty string, or zero, I don’t much care. All those things should rejected. There’s no need to use some sort of identity operator when _I can safely say that anything falsy is no good to me_ .

    function echoProperName($str = null) {
    	if($str != null) {
    		echo ucwords($str);
    	}
    }

I recognize I don’t even need to use comparison operators at all in this example, yet this very usage I have seen flamed for not using the identity operator. Ridiculous. It doesn’t fit here.

Programming languages give us loads of expressive tools, but by some sort of “convention”, we get told not to use many of them. **You can smash your thumb with any tool**. No need to tell me not to use ’==’ or ’++’ or some such because some people have hurt themselves with it.

