---
layout: post
title: 'Palindromic Word: Answer'
date: '2009-02-19T11:33:00-08:00'
tags:
- javascript
tumblr_url: https://seanmonstar.com/post/706935356/palindromic-word-answer
---
Earlier this week I posed a simple programming question on how to determine if a string is a [palindrome](http://en.wikipedia.org/wiki/Special:Search/palindrome) . That was the first interview question I’ve recieved in my career, and wanted to share it. Now I’ll share my answer that I gave back then.

Disclaimer: This interview was for a Mobile Java development position, so when I was asked the question, I was told to use basic constructs, cause everything is expensive on a piece-of-crap phone.

#### The Thought Process

A palindrome is when a word or sentence is the same spelt backwards, so I first thought I should reverse the string and compare the results. But not only does this take the time to reverse the string, it also allocates memory for 2 string objects, which I really don’t need.

So then I decided to do a character by character comparison, comparing the first character with the last, and so on. Not too bad, a simple for loop really. And to cut the process in half, you only really need to compare half of the string, since the other half is the reverse. If the first halves match, it **will** be a palindrome.

There’s an extra step in this function in the beginning, since I felt it’d be nice to compare sentences as well. Originally I didn’t have to, and didn’t need to process a regular expression.

#### isPalindromic: in Javascript

    function isPalindromic(string) {    
    	var trimmed = string.replace(/[^\\w]/g,'').toLowerCase();
    	for(var i = 0; i < trimmed.length / 2; i++) {
    		if(trimmed[i] != trimmed[trimmed.length - 1 - i]) {
    			return false;
    		}
    	}
    	return true;
    }

#### Who cares?

The developer doing the interview gets to watch you write on a whiteboard, and see you plan out your function. So in my case, he sees that I’ve got the concept down first: comparing the result backwards, and then that I realize it can down without two strings. And he also gets to notice whether or not I know that I only need the first half, or if I’ll leave slightly un-optimized.

Granted, this is a small function, and in many applications, such optimization won’t make or break the program. Also, my position **required** all optimizations possible to fit a playable game in the few kilobyes that the phone had to offer. Regardless of those, the thought process was the most important part.

The way one develops a single function reflects how he will develop an application. Does the programmer understand the language, and able to use its stengthes while minimizing weaknesses? Outside the language, is the developer able to shorten the work required to solve any problem? And then shorten it again? Will he be making good functions from the start?

