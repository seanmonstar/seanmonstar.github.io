---
layout: post
title: A Basic Lesson in Password Hashing
date: '2009-07-01T09:30:00-07:00'
tags:
- security
- nodejs
- bcrypt
- scrypt
tumblr_url: https://seanmonstar.com/post/707158385/a-basic-lesson-in-password-hashing
---
Update: May 11, 2014.

It’s quite easy now to hash passwords safely. Just use [bcrypt](https://www.npmjs.org/package/bcrypt) or [scrypt](https://www.npmjs.org/package/scrypt). scrypt is likely to be more secure, but as it’s also newer, it’s had less time for people to find weaknesses.

Example hashing with bcrypt:

    var bcrypt = require('bcrypt');
    bcrypt.hash(password, bcrypt.genSaltSync(12), function(err, hash) {
        // hash is the hashed text, store it in db
    });

And then when a user logs in, you can compare it like this:

    var bcrypt = require('bcrypt');
    bcrypt.compare(password, hash, function(err, isMatch) {
        // isMatch is a boolean if the password matches
    });

