---
layout: post
title: Guard and Default Operators of JavaScript
date: '2008-12-24T14:48:00-05:00'
tags:
- javascript
- bestof
tumblr_url: https://seanmonstar.com/post/707078771/guard-and-default-operators
---
Programming languages with C-style syntax all have logical operators for AND, OR, and NOT. They are &&, ||, and !, respectively. In Javascript, the way the languages determines logical operations and the values Javascript treats as true or false lead to people using the AND and OR operators for _guard and default situations_ . Let me explain.

#### Guard and Default? How so?

Here’s a quick look at how you can guard a value:

    var getUsername = function() { return loggedIn && username;}

If the user hasn’t logged in yet, `loggedIn` will be false, and irregardless of what `username` might have been set to, our function will return false. If `loggedIn` is true, then it will proceed to the next variable, and return `username` .

In short, A && B will return B if A is true (what Javascript considers true), otherwise it will return A.

And here’s quickly what defaulting looks like:

    var MGFX = MGFX || {};

I do this at the top of some of my Javascript classes. I use MGFX as the namespace to hold all my classes, and to make sure it always exists, I declare this at the top. If I’ve declared the var MGFX before, then the new MGFX will be the old one. If not, it will be an empty object.

In short, A || B will return A if A is true, otherwise it defaults to B.

#### Wait, those are logical operators, though…

Indeed, when I first started using these behaviors, I found it pecular how this was possible. Don’t get me wrong, it’s nice being able to declare a default in 1 line instead of 4. But reading it makes me want to think _it should be returning a boolean_ .

So to quench my curiosity, I opened up Firebug this morning and played with all sorts of possibilities, finally understanding why this actually makes sense.

##### Javascript True and False

It’s worth knowing first what is considered true and false in Javascript.

All these will evaluate to true (ie if you put them in an `if` statement):

- true
- All numbers besides 0
- All non-empty strings (includes “false” and “0”)
- An array (including empty: [])
- An object (including empty: {})

And these are the false values:

- false
- 0
- “”
- null
- NaN
- undefined

##### Why This Makes Sense

To understand what is happening, you need only to realize what an AND or OR operator does. Let’s take AND. In order for an AND operation to return true, both parts of the AND must individually be true. So, in Javascript, the first value is evaluated, and only if it is true, does it bother to evaluate the next value. Makes sense, since the second value doesn’t matter if the first is false.

Well, then, really, you could get away with returning the evaluation of the first variable if it was false for the whole AND statement. And if it were true, the AND statement now depends on the evaluation of the second statement and cares nothing about the first (since its true). Now returning the second evaluation would be the correct answer.

The same can be applied to OR. If the first statement is false, then the result of the OR hangs entirely on the second statement. And if the first statement is true, it doesn’t need the second statement for the OR to be true.

The only confusing part after realizing this, is the values Javascript determines is true. Or rather, that _it basically doesn’t coerce the evaluations into booleans. It leaves them as is._

So: 0 || “apple” returns “apple”, because it evaluates to true, and thats enough for Javascript.

##### Achieving the Original Results

I like the way this works. But I did want to see if I could get the original results in cases where I want a boolean of the result. The only way I could see doing this is using a double NOT. The NOT (!) operator in Javascript will **always** return a boolean. So using !! will give me the boolean of the opposite, and then the boolean again of the true result. One last example:

    var iAreABoolean = !!(14 && "sean"); //returns !!("sean") which returns !(false) in turn gives me true.

