---
layout: post
title: 'MVC in MooTools: Views'
date: '2010-09-02T12:39:00-04:00'
tags:
- mootools
- mvc
- monstars.js
tumblr_url: https://seanmonstar.com/post/1053909313/mvc-in-mootools-views
---
<small>This is another installment about how my <a href="http://github.com/seanmonstar/monstars.js">MooTools MVC framework</a> rocks. Check it out. If you’re interested in the first part, check out my write-up on using <a href="http://seanmonstar.com/blog/2010-08-25-mvc-in-mootools-models/">Models</a>.</small>

After you make the building blocks of your application, using [Models](http://seanmonstar.com/blog/2010-08-25-mvc-in-mootools-models/), Views are how you display that information to a user. Having written a lot of JavaScript that adds content to the page, I know just how much it can suck. You typically have two options:

1. Concatenate a huge string of markup with variables, then setting the `innerHTML` of a `div`.
2. Use a bunch of `document.createElement` and `el.appendChild`, ad infinitum.

Both are equally unappetizing. Wouldn’t it be so much nicer to write your views like we do in other languages?

### Templating

Here’s a nice mix of HTML and JavaScript. No tricky string concatenation<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>. You just have to put your JavaScript inside tags like you would in PHP.

    <ul id="Tasks">
    <% tasks.forEach(function(task) { %>
    
        <li class="task">
            <input type="checkbox" />
            <%= task.get('title') %>
            <% if(tag.get('tags').length) { 
                print(view('tasks/inline_tags', { tags: task.get('tags') }));
            } %>
        </li>
    
    <% }); %>
    </ul>

### View.Helpers

Besides `print`, which does exactly what you think it does, there are a bunch of other functions that are sort of “globally” accessible from a View. And these are functions defined on `View.Helpers`. The most basic version of the helpers only includes a `view` function, which let’s you nest views inside each other, like I used above. If you include the `View.Helpers` file, you get more functions, such as `excerpt` (truncates to a certain length) and `date` (allows date formating similar to PHP’s [date](http://php.net/date) method). As I find more basic functions that you would want in a templating system, I’ll add more, such as possibly something like `escape`.

### The File

These are saved as `.html` files. You don’t need to manually include them like Models or Controllers; the View class will first check to see if they’ve been included, and if not, go and fetch them over the wire. And they go in you app’s `views` folder. The example above would have a structure like so:

    js/
        views/
            tasks/
                list.html

### The View Class

The `View` Class is what converts templates into views for your application. You can instantiate a View, and then pass it data and ask for it to process itself using the passed data.

    var myView = new View('tasks/list').render({ tasks: myTasks });
    myEl.grab(myView);
    myOtherEl.set('html', myView);

Views have both a [`toElement`](http://mootools.net/blog/2010/03/19/a-better-way-to-use-elements/) and `toString` function defined to return the processed mark-up in the proper format. This means you can simply pass the view as a parameter to most MooTools Element methods, and it just works<sup id="fnref:2"><a href="#fn:2" class="footnote-ref" role="doc-noteref">2</a></sup>.

Good news, though: Controllers have a convenience method called `view` that really makes this a joy to use. I’ll have more to say about Controllers in my next report.

* * *

1. 

The string concatenation is done internally. Everything in the template is turned into a string using [John Resig’s micro-templating](http://ejohn.org/blog/javascript-micro-templating/) suggestion.&nbsp;[↩︎](#fnref:1)

2. 

Regular DOM methods might not though. `appendChild` expects an Element, and since it’s not part of MooTools, it’s not rigged up to called `toElement`. Also, because of the way `toString` works with Classes in Internet Explorer, this feature doesn’t work in those browsers. MooTools 1.3 repairs that problem.&nbsp;[↩︎](#fnref:2)

