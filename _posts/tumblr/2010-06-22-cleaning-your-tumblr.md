---
layout: post
title: Cleaning Your Tumblr
date: '2010-06-22T13:53:55-04:00'
tags:
- javascript
tumblr_url: https://seanmonstar.com/post/726009563/cleaning-your-tumblr
---
I originally setup my Tumblr unsure of what to use it for, so I just added a bunch of feeds to it, like my Twitter, my blog feed, my delicious links, and my github activity. But when I decided to actually use my Tumblr as a real blog, I decided I didn’t want all that junk in here.

For whatever reason, Tumblr doesn’t provide any sort of simple way to clean out a lot (_1000+_) of posts. The only way to delete posts it to scroll to the post in the dashboard, click the “delete” button, and wait for the `POST` to be submitted and the page reloaded, which forgets your current scroll position. I wasn’t going to do that 1000 times.

Thankfully, there’s a decent API that lets you easily receive all your posts. The posts have a field set if they had been imported through a feed. And with the dashboard using [Prototype](http://prototypejs.org), it was a matter of putting together 2 functions to read and delete posts, and then some minor babysitting to make sure to pause when Tumblr would throttle me.

    var keepGoing = true,
        tumblr = '',
        email = '',
        password = '',
        posts = 0;
    function findFeedItems(json) {
        json.posts.each(function(p) {
            if(p['feed-item']) {
                apiDelete(p['id'])
            }
        });
    
        if (keepGoing) {
            apiRead.delay(20);
        }
    }
    
    function apiRead() {
        console.log('reading...');
        var nonce = new Date();
        var s = document.createElement('script');
        s.src = "http://"+tumblr+".tumblr.com/api/read/json?num=50&callback=findFeedItems&start=" + posts + "&nonce=" + (+nonce);
    
        document.body.appendChild(s);
    }
    
    function apiDelete(id) {
        console.log('deleting: %s', id);
        new Ajax.Request('/api/delete', {
            method: 'post',
            parameters: 'email='+email+'&password='+password+'&post-id=' + id,
            onSuccess: function() {
                console.log("Successful deletion: %s", id);
            },
            onFailure: function(transport) {
                console.log('delete failed: ', transport.status);
            }
        });
    }

While on the [dashboard](http://www.tumblr.com/dashboard), copy and paste this into your console, with the proper variables filled in, of course.

Just called `apiRead()` to get it started. When you see a delete failure, you can pause your purge by setting `keepGoing` to false. Wait a few seconds, set it back to true, and call `apiRead()` again.

Once you’re no longer seeing successes or failures, but you know theres more posts, you can set the `posts` variable to 50, or 100, or higher. That will start the search farther down your posts.

**Again, this script will delete everything imported from a feed.** If you want to just delete _everything_, pull out the `if(p['feed-item'])` condition, and just pass all the posts to `apiDelete`.

