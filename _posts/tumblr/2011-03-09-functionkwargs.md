---
layout: post
title: Function.kwargs
date: '2011-03-09T14:00:00-05:00'
tags:
- javascript
- mootools
- python
- planet
tumblr_url: https://seanmonstar.com/post/3746460491/functionkwargs
---
[Function.kwargs](http://jsfiddle.net/seanmonstar/WfWhN/)  

I get to write a lot of Python now on a daily basis, and some Python bits bleed back into my JavaScript writing. One aspect that I love is keyword arguments.

    def example(name, description, url, help, error):
        pass
    
    example(name='A', url=myURL, description='An example')

Having something similar in JavaScript is possible, but requires a bit of work for a function to support it. Inspired by Python and MooTools overloadSetter, I cooked up [a decorator](http://jsfiddle.net/seanmonstar/WfWhN/) to easily convert a function to allow keyword arguments<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>.

    var example = function(name, description, url, help, error) {
    
    }.kwargs();
    
    example({ name: 'B', error: true, description: 'kwargs in javascript' });
    example('C', 'args plus kwargs', { help: true, error: false });

Just like in Python, you can provide arguments the regular way, or pass in keyword arguments. You can also mix them; the last argument will be checked to see if it’s a kwargs-like object.

* * *

1. 

MooTools required, of course.&nbsp;[↩︎](#fnref:1)

