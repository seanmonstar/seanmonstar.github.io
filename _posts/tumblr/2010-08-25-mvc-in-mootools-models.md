---
layout: post
title: 'MVC in MooTools: Models'
date: '2010-08-25T11:00:00-04:00'
tags:
- javascript
- mootools
- mvc
- monstars.js
tumblr_url: https://seanmonstar.com/post/1009142033/mvc-in-mootools-models
---
It’s really bugged me when writing large JavaScript applications, organization of code never really seems to be considered. We use frameworks for all the server-side stuff we do, but everyone seems content writing JavaScript in [one big mess](http://jquery.com).

Granted, that’s why I was attracted to MooTools in the first place: the idea of organizing your code is built into the framework. If you don’t predominantly write classes with MooTools, you might as well be [writing jQuery](http://jqueryvsmootools.com). But even with classes, in a JavaScript application[^1] you write a lot of code that works with those classes and fiddles with the DOM. It can get to be a real _mess_.

A year ago, I got fed up with it. I was also interested in Adobe AIR at the time, and liked the idea of being able to copy over as much of my code as possible to make an AIR version of an application. Hopefully, the only changes would be the way data was stored and received. I feel pretty good when using the MVC paradigm on the server-side, and I felt it made sense in the browser as well. So I started a [MooTools MVC framework](http://github.com/seanmonstar/monstars.js).

### Models

I started with making various sub-classes of Model[^2] that interacted with specific data storage concepts. In Adobe AIR, a SQLite database would likely be used. In the browser, you’d likely have you’re data stored on the server, and need to access it via ajax. I wanted to be able to have the same model class, and just change its subclass based on need. Certainly, the `Model.SQL` could be used in newer browsers, as well, if you wished.

It didn’t take look for me to give up for the time being on AIR, but I still felt that the organization that such a framework provided was beneficial. Therefore, I spent more time on `Model.Ajax` and `Model.Browser`, since most of my use of the framework has been for the browser. `Model.Browser` uses a combination of localStorage, userData, and cookies, depending on support, to allow a web application to function entirely on the client side. I did much of this just to allow myself to work out an example application, without needing a back-end.

`Model.Ajax` is what I use the most, now. Let me show you an example model:

    var Task = new Class({
    
        Extends: Model.Ajax,
    
        fields: {
            id: Model.Fields.AutoField({ write: false }),
            title: Model.Fields.TextField(),
            created_at: Model.Fields.DateField(),
            tags: Model.Fields.ManyToManyField(Tag)
            is_done: Model.Fields.BooleanField({ default: true })
        }
    
    });

By extending `Model.Ajax`, `Task` will make some default assumptions about the back-end API. For instance, `Task.find` will call `/tasks/find`, `Task.get(5)` will call `/tasks/5`, and `myTask.save()` will call `/tasks/insert` or `/tasks/5/update` depending on if it’s new or not. And of course, because of the asynchronous nature of all these function calls, a callback can be passed, which will receive the instances as it’s only parameter upon completion. Such as:

    Task.get('123', function(task) {
        console.log(task); //the returned data, cast as an instance of Task already
    });

A model also declares its fields, so that every time a new Task is made with a hash of properties, it only extracts the ones it cares about, and converts them to the proper type if it can. Specifically, I’ve set `id` to read only, since I don’t want something accidentally changing the `id` of a task and sending it.

An instance of a `Model` can access it’s properties through getter and setter functions. These are similar to the [Element getter and setters](http://mootools.net/blog/2010/06/10/setting-up-elements/), and they allow each individual field type to cast to the proper type, or return a default. For example:

    var myTask = new Task;
    myTask.set('is_done', 'true');
    console.log(myTask.get('is_done')); //will return the boolean true, not the string.

This also allows Fields that provide relationships to other models cast ids to instances and back again. `Task.tags` will store an array of tag ids internally, and submit those to the server upon a save. But if you `get` the tags, the `ManyToManyField` will convert them into the existing `Tag` instances.

The point of using Models in this framework is to have a single location that is responsible for abstracting data retrieval[^3], and providing a central place to handle any data conversion. You shouldn’t have any Ajax requests or data validation any where else in your application.



[^1]: From here on out, my use of a JavaScript application is one where a large amount of it is written in JavaScript. Sure, your requests still need to interact with a server-side technology, but on the client side, most of the application functions without page refreshes.

[^2]: The assumption is made that you know what MVC is. If not, [read up](http://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller).&nbsp;[↩︎](#fnref:2)

[^3]: Whether the Model or the Controller should do the ajax requests has been [argued](http://www.alistapart.com/articles/javascript-mvc/) both ways when used in a JavaScript MVC framework, . Since the primary reason for a Model is to abstract the data storage system, it makes sense to me that only the Model should know if it needs to access a web server via ajax, or use localStorage, or use SQLite.&nbsp;[↩︎](#fnref:3)

