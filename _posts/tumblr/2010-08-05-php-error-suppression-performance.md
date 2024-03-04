---
layout: post
title: PHP Error Suppression Performance
date: '2010-08-05T16:22:00-04:00'
tags:
- performance
- php
- bestof
tumblr_url: https://seanmonstar.com/post/909029460/php-error-suppression-performance
---
Sometimes, a function might cause a warning, like when you’re messing around with files. It might be tempting to just prepend an [`@`](http://us2.php.net/operators.errorcontrol) to the function, and live on. But being curious, I researched if this does much harm besides being sloppy/lazy. It turns out there are performance costs for doing so as well.

I first built a simple test that would loop a million times accessing a variable with and without the suppression operator prepended. The differences were small, yet noticeable. Using the suppression operator ended up **taking 40% longer to execute**. Interesting, but then an [article by Vegard Andreas Larsen](http://vega.rd.no/articles/php-performance-error-suppression) pointed out something I failed to test:

> [The] assertion that it is the act of the @ operator that is very slow, is wrong. It is in fact the actual triggering of the error or warning by itself.

His tests show that while the suppression operator does add a little overhead, when an actual error[^1] occurs, you see a bigger cost. When using the suppression operator, you’re writing in a style that let’s you cause errors and not care, which decreases performance. The same thing applies to setting `error_reporting` to ignore notices or warnings. Just because their ignored, doesn’t mean PHP doesn’t try to throw them first.

A common example is when checking for properties of an object. When ignoring notices, you might do something like this:

    if($obj->prop) { 
        do_stuff($obj->prop); 
    }

If the property is undefined, a notice will be thrown, and then ignored. Performance penalty. Turns out that [`isset`](http://us.php.net/isset) is quite important, after all. Another instance could trying to call `file` without first calling `is_file`. I used to think, _So what, it’s a dynamic language, it’ll be just fine._ Now, I’ll be littering my code with `isset` everywhere.

Just to be sure, I altered my original test to check the differences between using a conditional test with isset versus just suppressing the notice. The difference was that suppressing the notice (a **notice!** ) took **100%** as long as just checking if it existed first. Try it yourself.

    <?php
    
    function no_suppress() {
        $a = 0;
        $b = new stdClass;
        $a = (isset($b->asdf) ? $b->asdf : null);
    }
    
    function suppress() {
        $a = 0;
        $b = new stdClass;
        $a = @$b->asdf ? $b->asdf : null;
    }
    
    function do_test($suppress = false, $loops = 1000000) {
        if($suppress) {
            echo "starting suppress...\n";
            $start = microtime(true);
            for($i = 0; $i < $loops; $i++) {
                suppress();
            }
            $end = microtime(true);
        } else {
            echo "starting no_suppress...\n";
            $start = microtime(true);
            for($i = 0; $i < $loops; $i++) {
                no_suppress(true);
            }
            $end = microtime(true);
        }
        echo "ended: " . ($end - $start) . "\n";
    }

Now, this might not be the biggest thing in the world. But it’s enough for me to change my ways, since it affects me once to write it, and my users an infinite amount of times having to execute. Since I believe in optimizing for users instead of developers, that’s how it’s going to be.



[^1]: I use the term error to mean any message thrown. It includes errors, warnings, notices, etc.

