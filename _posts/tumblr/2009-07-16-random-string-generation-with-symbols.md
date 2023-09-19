---
layout: post
title: Random String Generation with Symbols
date: '2009-07-16T13:22:00-04:00'
tags:
- php
tumblr_url: https://seanmonstar.com/post/706962239/random-string-generation-with-symbols
---
I’ve been playing with some random string generation, since I built a fairly simple one in a recent project for when users forget their password, and I reset it. It seemed decent enough: produced a string of strong size, alpha-numeric. **It was good enough.**

    base_convert(uniqid(rand(),true),10,36)

It didn’t take long after I had commited that code before I started thinking it could be better. At least, if I need it to be better, then it could be. For instance, if it’s going to be used for security reasons, it should include symbols, and upper-case characters. Then I thought that this “improved” version would be far superior for [providing a salt](http://mcarthurgfx.com/blog/article/a-basic-lesson-in-password-hashing) than previously seen examples.

    substr(md5(uniqid(rand(),true)),0,8)

The above line will provide a random string, but it’s weak in a few ways. First, it’s only 8 characters long. We should be aiming in the high ‘teens. But that’s easily fixed by changing the `substr` length. However, its bigger weakness is that it’s text coming from a hexidecimal output. Each character only has 16 possibilities, which is **way smaller** than 62, or more.

#### I Like “Or More”

The amount of possible combinations is dependent on the number of characters, and the number of characters available. In math terms it would look something like this:

    f(x,y) = x^y

Where x is the size of the character set, and y is the number of characters in the string. So, the possible variations of a string of length 15 hexidecimal characters is: `16^15 = approx. 1.15 quintillion` .

If we increase the character set to include all letters, upper and lower case, plus most all the symbols on the keyboard, we can get something like 92 characters to choice from. With the length staying the same, we get: `92^15 = approx 286 octillion` . Don’t worry, I did the math for you: That’s an _increase of 250 million times_ .

Like a computer really wants to brute force **that**.

#### How I Got “More”

I didn’t want to simply create a huge list of possible characters, and then use a loop and a random number generator to eventually build the string. I wanted to try to keep it as much as possible in native functions. I recognize that using a loop might provide more randomness, as there would be no possible pattern, but I feel this is sufficient.

I start with my original method, I just call that `randomAlphaNumeric` , cause that might be useful in other situations. Then, a whole mess of things.

I split the string based on a random number, then rebuild the string with spaces. I capitalize the first letter of every word, then remove all the spaces. Then I grab a couple of symbols based on a random length, attach them on the end, and shuffle the string. Last, I take the length of the original alphanumeric, simply to prevent hugely differing string lengths.

    function randomAlphaNumeric() {    
    	return base_convert(uniqid(rand(),true),10,36);
    }
    
    function randomPassword() {    
    	$alphanum = randomAlphaNumeric();    
    	$symbols = str_shuffle('~`!@#$%^&*()_-+={[]}|\\\\;:,<.>/?');        
    	return substr(           
    		str_shuffle(             
    			str_replace(' ','',              
    				ucwords(               
    					implode(' ',                
    						str_split($alphanum,                
    							rand(1,strlen($alphanum)-1)                 
    						)                
    					)               
    				)              
    			)             
    			.substr($symbols,0,              
    				rand(1, strlen($symbols) - 1)              
    			)           
    		),0,strlen($alphanum)-1);
    }

Again, this isn’t the best way of doing things. It was more of an exercise on my part to find an interesting way to generate a string that contained all the characters I wanted. We can easily remove some of the symbols from the `$symbols` string if any are illegal in the usage of the generated string.

