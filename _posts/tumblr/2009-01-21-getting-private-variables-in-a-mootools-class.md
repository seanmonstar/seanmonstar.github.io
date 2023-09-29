---
layout: post
title: Getting Private Variables in a Mootools Class
date: '2009-01-21T14:12:00-05:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/708687601/getting-private-variables-in-a-mootools-class
---
After a good read though [Javascript: The Good Parts](http://www.amazon.com/gp/product/0596517742?tag=mcgf-20), and the start of a rather large class that I’ll be writing in my spare time, I thought of how I could setup private variables in a Mootools class. [YUI’s Module pattern](http://yuiblog.com/blog/2007/06/12/module-pattern/) is an easy way to make some private variables, but it needed to fit into the Mootools Class pattern. So I merged the two ideas, and created a Class Mutator, that lets you define a Privates object, and you get private variables!

#### Class.Mutators.Privates

I had found a Privates mutator already written by Nathan White, but my testing revealed some shortcomings. You don’t need to understand anything of what I’ve done to use it, but I’ll show it for those who care:

    Class.Mutators.Privates = function(self, privates) { delete self['Privates']; var oldInit = self.initialize; self.initialize = function() { var tempThis = $unlink(this); var instPriv = $unlink(privates); var instance = oldInit.apply(tempThis,arguments); for(var prop in tempThis) { if(instPriv.hasOwnProperty(prop)) { instPriv[prop] = tempThis[prop]; } else { this[prop] = tempThis[prop]; } } var that = this; for(var key in tempThis) { if($type(tempThis[key]) === 'function' && key !== 'initialize') { (function (fn) { var oldProp = that[key]; that[fn] = function() { var my = $merge(that, instPriv); var bound = oldProp.bind(my,arguments); var returns = bound(); for(var prop in my) { if(instPriv.hasOwnProperty(prop) && $type(instPriv[prop] !== 'function')) { instPriv[prop] = my[prop]; } else { that[prop] = my[prop]; } } return returns; }; })(key); } } return instance; } return self; };

I delete the Privates object out of the class, and redefine that initialize function, so that way all this magic happens for each instance, whereas most Mutators affect the class itself, and the first way I did it had the Privates object became static in nature, and all instances shared the private variables.

I make a copy of the class object, just as initialize does in the Native class, and pass that into the class’ original initialize function. I then take that object and search for any properties that are supposed to be private, and update the private variables, and pass all other variables (meaning any new variables are public) to the public object. I also redefine every function of the class at initialize, and created `var` ’s of the private object, and since everything is defined in the same function, the scope is never lost. After execution of any function, all properties that are private are copied into the private object. Each function is called with the object that contains `this` merged with the private object, so all the private variables are accessible as this.secret in the functions, like you’d expect in any other programming language.

#### An Example: Secret

Here’s the class I wrote to do all my testing, and shows how everything works:

    var Secret = new Class({ Implements: [Options, Events], Privates: { secret: 'shhhh' }, open: null, initialize: function(word) { this.secret = word; this.open = 'not a secret'; }, getSecret: function() { return this.secret; }, setSecret: function(newWord) { this.secret = newWord; this.notSecret = 'im a new prop in this'; }, getOpen: function() { return this.open; } });

`getSecret` `open` is a public property of the Secret class. With a new instance, you’ll notice you can access open publicly, but not secret.

    var test = new Secret('this a secret test');//test.open//Reveals the open value //test.secret//Reveals undefined

Trying to set `test.secret = 'a new secret'` will set the secret property temporarily, but it doesn’t override the private one, and when accessed in a function, the private one will instead write over the public one, since I check if the property is supposed to be private, and strip it from the public object.

#### Notes

I doubt I did it the most effectively. After writing it, I tried refactoring it some by declaring a helper function that strips the private variables out of the public object, since I do that for loop twice. But doing so caused problems, and I wasn’t up to the task of learning what the new problems were. By all means, if you see optimizations, I’d love to know them, I’ll make an update, and I’ll learn as well.

#### Update (11/18/09)

This only works prior to MooTools 1.2.3. In 1.2.3, a new implementation of Class.js was introduced, and I don’t think this works the way I hoped in the newest versions. You can, though, now use a [protect decorator](http://seanmonstar.com/blog/protected-methods-in-mootools-classes/) on your functions.

