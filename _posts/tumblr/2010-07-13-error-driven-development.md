---
layout: post
title: Helpful Errors Messages Are Important
date: '2010-07-13T17:44:00-04:00'
tags:
- php
- usability
- error driven development
tumblr_url: https://seanmonstar.com/post/808058987/error-driven-development
---
If you write software, you write bugs. And after those bugs, there’s still errors that will happen in code you didn’t get to touch. Errors happen. That’s not new. And users know that things can go wrong sometimes. What pains me, is that we know&nbsp;exactly what went wrong, and we don’t translate that for the user.

### In Windows

It’s surprising to find how often you’ll find an error message that’s only helpful to the creator of the original system. So often as user, you can reliably create a certain error to happen, and the only message you get back from the program is something like `Error 0x2211108: Don't do that`. If we as software engineers would just take the time to make sure that every error shown to a user let them know what happened (if its useful, which it actually very often is), and **any steps they can take to fix the error**.

Some of the most terrible error message design is the infamous [blue screen of death](http://en.wikipedia.org/wiki/BSOD). It completely disrupts the computer (which could be acceptable, some fatal errors can’t be recovered from). But worse, it tells the user virtually **nothing** useful. Just a bunch of computer gibberish.

Disregarding how ugly the error looks, the Windows team could have at least intercepted the error and displayed a useful message. Here’s an example:

#### Original BSOD

![original BSOD](https://64.media.tumblr.com/tumblr_l5gxtvHjlR1qzek7l.gif)

#### Improved BSOD

![improved BSOD](https://64.media.tumblr.com/tumblr_l5gxu8c1HG1qzek7l.gif)

### In Blazonco

A while ago, I went through and documented many of the errors that errors that might bubble up in Blazonco. We’ve created a help page for each error, which depends on its unique error code number. I also tried to make sure that all errors are in a language the user understands. Error codes mean nothing to most people, so we don’t display them. They’re only in the link that the user can click to learn more about[^1]. They don’t need to see that I tried to reference a property of a null object, or that file system threw an error about permissions. Instead, I tried to catch all those, and throw a much _prettier_ error.

![cryptic SQL error](https://64.media.tumblr.com/tumblr_l5imtxrjna1qzek7l.jpg) ![user friendly error](https://64.media.tumblr.com/tumblr_l5imu51ykn1qzek7l.jpg)

We basically have a `try/catch` at the top most function of our application, which will try to show the error message in a graceful way (inside that little error popover). But it’s not in that top function where we convert gibberish into English. It’s done whereever a proper error could happen.

    $post = new BlogPost;
    //...
    try {
        $post->save();
    catch (PDOException $ex) {
        if($ex->getCode() == UNIQUE_CONFLICT_ERROR) {
            throw new BlogPostException('You already have a blog post with that URL. You will need to use a different one so people can view your post.', 40006023);
        }
    }

We still throw an exception, because an error did exist, but we can show much more meaning knowing that it’s a `BlogPostException`, and the message is much more meaningful to the user than what the SQLSTATE would have said.



[^1]: We also [log](http://www.codinghorror.com/blog/2009/04/exception-driven-development.html) all errors that occur. That way, each morning, I can open up our Superadmin and see if a user has ran into an unexpected internal error, or if a lot of users are receiving an “excepted” error too often, suggesting a flaw in the UI.

