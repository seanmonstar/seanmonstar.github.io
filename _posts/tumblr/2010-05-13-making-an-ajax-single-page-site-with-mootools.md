---
layout: post
title: Making an Ajax Single-page Site with MooTools
date: '2010-05-13T18:27:00-04:00'
tags:
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/709072388/making-an-ajax-single-page-site-with-mootools
---
The past couple weeks I’ve been working on a website that, with JavaScript enabled, will never refresh the page[^1]. With MooTools, I just needed to manipulate the URL hash and use `Request`, making it fairly easy to get an Ajax site that works like Facebook. Granted, this has nothing about handling CSS or JavaScript assets for each page. I’m currently handling that myself, and perhaps I’ll share my findings for that another time.

#### Use Event Delegation

First of all, I started with making every link in the site need to trigger the Ajax loading. MooTools More has an excellent [Event Delegation package](http://mootools.net/docs/more/Element/Element.Delegation). Using that, I’ll list for every click on a link on the page, check if it’s to the same domain, and load those pages asynchronously, otherwise just let the browser travel to the external page.

    $(document.body).addEvent('click:relay(a)', function(e) {
        var href = this.get('href');
        if(isSameDomain(href)) {
            e.preventDefault();
            window.location.hash = href;
        } else {
            //let the browser do its thing...
        }
    });

#### Change the Hash to provide History

We change the hash around because in most browsers, this will register another history location, allowing the forward and back buttons to work. It also allows someone to copy and paste a link. [IE6](http://seanmonstar.com/blog/it-s-all-your-fault/) and 7, of course, don’t. For them, we manipulate an `iframe`, which _will_ register history locations.

Most browsers provide a [hashchange event](https://developer.mozilla.org/en/dom/window.onhashchange), but again, IE6 and 7 fail to deliver. Thankfully, [Matias Niemelä has added support for the hashchange event into MooTools’ Event system](http://github.com/matsko/Mootools-window.onhashchange), browser compatibility and all. If you’re curious, you can [read up on the different browser issues his effort solves](http://www.yearofmoo.com/onhashchange/).

Once I got that added to the page, and setup a listener, the listening function will run everytime a user clicks a relative link, since we change the hash in our click handler above.

    window.addEvent('hashchange', function(hash) {
    new Page(hash).load();});

#### Make the Request and insert the Page

I actually use a `PageManager` arbiter to handle `Page` loading, inserting, removing, and caching, but that isn’t necessary (though I recommend it).

To get you started, the `load` and `insert` methods of the `Page` class are shown below:

    var Page = new Class({
        Implements: [Options, Events],
        options: {
            container: 'Content'
        },
        initialize: function(url, options) {
            this.setOptions(options);
            this.url = url;
        },
        load: function() {
            var that = this;
            this.request = new Request({
                url: url,
                onSuccess: function(text) {
                    that.content = text;
                    that.insert();
                },
                onFailure: function() {
                    that.content = ERROR_404_MSG;
                    that.insert();
                }
            }).send();
            return this;
        },
        insert: function() {
            //no request means we've never called load()
            if(!this.request) {
                return this.load();
            } 
            //dont insert until we're done loading
            if(this.request.running) {
                return this;
            }
            $(this.options.container).set('html', this.content);
            this.fireEvent('insert');
            return this;
        }
    });

I’ve also declared methods like `remove` and `unload`, which my `PageManager` calls. I’ll be experimenting with what kind of optimizations I get by removing the content into a `documentFragment` instead of disposing of the content rebuilding if the user clicks back. But that should help give a start to trying to do this yourself.



[^1]: As a precursor, its very simple to make a website like this work without the Ajax, so that its still accessible and searchable. In your server side code, most JavaScript frameworks will send an extra header, `X-Requested-With`, with the value of `XMLHttpRequest`. You can check if that header has been sent, and if so, send only the content html, and if not, send the entire page instead.

