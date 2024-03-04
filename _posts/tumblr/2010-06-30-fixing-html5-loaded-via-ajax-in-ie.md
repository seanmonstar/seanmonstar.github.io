---
layout: post
title: Fixing HTML5 Loaded Via Ajax in IE
date: '2010-06-30T14:24:18-04:00'
tags:
- html5
- javascript
- mootools
tumblr_url: https://seanmonstar.com/post/754502783/fixing-html5-loaded-via-ajax-in-ie
---
Internet Explorer doesn’t support HTML5 elements. You must provide a [shim](http://www.google.com/search?q=ie+html5+shim). But this only helps fix any markup that is in the original document. If you receive more markup via an ajax request, and want to insert it into the document, IE has a hissy fit. You can’t stick HTML like that into some elements `innerHTML` property and hope it parses correctly.

What I did instead, was before inserting the HTML, I’d find all HTML5 elements[^1], convert them to spans with an extra attribute `htmlfix` with the value set to the tag name it should be. Then after inserting into an `innerHTML`, I’d find all the spans with an `htmlfix` attribute, create the proper element, transfer all its `attributes`, set its `innerHTML`, and `replaceNode`.[^2]

    function processAjax(content) {
        var spanned = content,
            html5els = ['nav','header','article','footer','section','time'], //etc...
            attributes = ['id', 'class', 'pubdate', 'datetime']; //etc...
    
        html5els.forEach(function(tag) {
            var re = new RegExp('<(\/?)'+tag+'([^>]*)>', 'g');
            spanned = spanned.replace(re, '<$1span htmlfix="'+tag+'"$2>');
        });
        var el = new Element('div', { html: spanned }).getFirst();
    
        //after IE parses the spans, createElements of all the "fixed" elements
        el.getElements('span[htmlfix]').forEach(function(span) {
    
            var tagname = span.get('htmlfix');
            span.erase('htmlfix');
    
            var newEl = new Element(tagname, { html: span.innerHTML });
    
            //transfer attributes
            attributes.forEach(function(attr) {
                var val = span.get(attr);
                if(val) {
                    newEl.set(attr, val);
                }
            });
    
            newEl.replaces(span);
        });
    
        return el;
    }



[^1]: Yes, I used [Regex to parse HTML](http://stackoverflow.com/questions/1732348/regex-match-open-tags-except-xhtml-self-contained-tags/1732454#1732454). No, it didn’t create a rift in space. [I knew what I was doing](http://www.codinghorror.com/blog/2009/11/parsing-html-the-cthulhu-way.html).

[^2]: This uses [MooTools](http://www.mootools.net), because it’s awesome. I wasn’t feeling very masochistic this time around, so I didn’t create a vanilla JS version.&nbsp;[↩︎](#fnref:2)

