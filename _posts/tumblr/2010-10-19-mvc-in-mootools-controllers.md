---
layout: post
title: 'MVC in MooTools: Controllers'
date: '2010-10-19T02:33:00-04:00'
tags:
- javascript
- mootools
- mvc
- monstars.js
tumblr_url: https://seanmonstar.com/post/1349631987/mvc-in-mootools-controllers
---
<small>This is another installment about how to use my <a href="http://github.com/seanmonstar/monstars.js">MooTools MVC framework</a>. If you’re interested in the previous parts, check out my write-up on using <a href="http://seanmonstar.com/blog/2010-08-25-mvc-in-mootools-models/">Models</a> and <a href="http://seanmonstar.com/blog/2010-09-02-mvc-in-mootools-views/">Views</a>.</small>

It’s often explained that Controllers are how you tie Models and Views together. I don’t want to get into _that_ argument. For me and my framework, Controllers handle DOM events. They listen to the views for events, update models, and then render new views. Let’s take a look.

### Event.Delegation

[Event delegation](http://mootools.net/docs/more/Element/Element.Delegation) is used throughout the controllers, since it greatly eases the pain of changing the DOM around all the time. When you name a controller, the first part of the name will be used to find or create an element to look over, unless another element is specified. Example time:

    var RecipesController = new Class({
        Extends: Controller
    });
    
    //attaches itself to <div id="Recipes"></div>
    //if that doesn't exist, it gets created

The rest of the controller contains event handlers for everything that goes on inside that element, and methods that various event handlers can call.

    var RecipesController = new Class({
        Extends: Controller,
        events: {
            load: function() {
                Recipe.findAll(this.list.bind(this));
            },
            'click:relay(.recipe .delete)': function(e) {
                e.preventDefault();
                var recipeView = $(e.target).getParent('.recipe');
                recipeView.get('model').destroy(function() {
                    recipeView.destroy();
                });
            }
        }
    });

The load event will fire on domready, and eveything else is in the MooTools event delegation way. Currently, all event handlers are bound to the controller instead of the element, since it’s easy enough to get the element from the event object, and you’re most likely going to want to call a method of the controller. For instance, you might notice that the `load` event has a call to `list` on the controller.

### this.view

First-class methods on a controller get access to the `view` method. This is a convenience method that will instantiate [views](http://seanmonstar.com/blog/2010-09-02-mvc-in-mootools-views/) for you, with a little auto-magic thrown in.

    var RecipesController = new Class({
        Extends: Controller,
        // events: {},
        list: function(recipes) {
            $(this).set('html', this.view({ recipes: recipes }));
        }
    });

`view` can accept 2 parameters: the view name, and a data object. However, you can exclude the view name, and it will try to use the controller and method names. In the `list` method above, since no view name is passed to `view`, it will try to load `recipes/list`.

### That’s About It

That’s all there is to Controllers, actually. They just handled events fired by Views, let appropriate Models know, and change the View if necessary. Although, while the role seems small, it tends to contain the majority of logic anyways. Models just hold on to their data, and Views just show it, but Controllers are how you manipulate it.

