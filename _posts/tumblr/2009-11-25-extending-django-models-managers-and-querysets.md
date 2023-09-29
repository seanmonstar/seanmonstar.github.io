---
layout: post
title: Extending Django Models, Managers, And QuerySets
date: '2009-11-25T11:10:00-05:00'
tags:
- python
- django
- bestof
tumblr_url: https://seanmonstar.com/post/708862164/extending-django-models-managers-and-querysets
---
In a recent pet project, I’m exploring Django. As I’m used to in our PHP framework, [I like to extend Models](http://seanmonstar.com/blog/automagic-prefixes-for-model-fields/) with methods that a model should keep contained, and then I can call multiple times elsewhere in the Controller <ins>View in Django (don’t start me on the stupidity of the naming scheme)</ins>. In PHP, it’s a bit more straight forward: You can simply write some new functions inside the class. In Django, it was a little more complicated. I explored several different parts that all affect writing methods that should be contained in the Model area of the application.

#### Models

First, Models. You can simply write some methods in the Model class definition, the same way you’d like to in PHP. A difference though, in Python we don’t get function decorators like we do in PHP. In PHP, I would write instance methods that manipulate an object, or instance, of the Model. Such as `$ball->explode()`. I would write static function that manipulate the table of models, such as `Ball::get_exploded()`.

In a Django Model, the methods we define are only there to manipulate instances of the Model (in most cases). For example:

    class Ball(models.Model):
    	def explode(self):
    		self.exploded = true;
    		self.lifetime = datetime.now() - self.created_at

We would use this elsewhere to make sure that when the Ball explodes, we also record how long the ball was inflated.

    ball = Ball.objects.get(id=1)
    ball.explode()
    ball.save()

#### Manager

The Manager is how we access the table. It’s largely like the static methods we might use in PHP. The default property to access the manager is `objects`.

Managers provide a good set of methods to select and filter the objects you want to receive. However, I started to notice certain trends in the functions I would use, receive a certain group of objects. Naturally, moving those combinations of methods, plus complicated `extra` calls, into their own methods is good for [DRY](http://en.wikipedia.org/wiki/DRY). I’ll use a simple example for now:

    class BallManager(models.Manager):
    	def get_exploded(self):
    		return self.filter(exploded=True)class Ball(models.Model):
    	objects = BallManager()

Now, we get a new method to access all exploded Balls in the database.

    Ball.objects.get_exploded()

This is supremely more useful when you start making complicated queries, in several different views. Let me just show you an real example in my pet project:

    def due_this_week(self):
    	return self.extra(where=["due_date > now() - interval '1 day'", "due_date < now() + interval '7 days'", "not(due_date isnull)"])

This insures that if I find my query to be slightly buggy, or if I want to pad it an extra day, I only have to change my method. The benefits should be obvious enough. Manager methods usually return QuerySets, so let’s see why extending the QuerySet is also useful.

#### QuerySet

By adding our method to the manager, you can call it from the Manager property, but we might want to use our methods later on in a query. Currently, the method is only available from `Ball.objects.get_exploded()`.

By adding the methods to the `QuerySet` for `Ball`, we can use `get_exploded()` after a `filter().filter().extra()`.

However, adding a method to the `Manager` and also the `QuerySet` the `Manager` uses would mean writing the method twice.

#### QuerySetManager

Doing some searching, I found the [QuerySetManager](http://www.djangosnippets.org/snippets/734/), a snippet someone had put together that allows us to add methods to both the QuerySet and Manager at the same time. We define the QuerySetManager, and then tell the model use that for its Manager. Then, we can define a QuerySet inside the Model declaration, since Python classes allow you define inner classes.

Here we go.

    class QuerySetManager(models.Manager):
    	def get_query_set(self):
    		return self.model.QuerySet(self.model)
    	def __getattr__ (self, attr, *args):
    		return getattr(self.get_query_set(), attr, *args)

The Manager.get\_query\_set is the function that gets called internally whenever it needs to retrieve a query set of the manager’s models. By overwriting it, we can return a different QuerySet, one we extend to have new methods.

Defining \_\_getattr\_\_ is like [defining magic functions in PHP](http://seanmonstar.com/blog/overloading-objects-in-php/): any attribute (read: method or property) that doesn’t exist, will try the \_\_getattr\_\_ method, before raising an AttributeError. This lets us write all the methods on the QuerySet, and then any method we call on the Manager, will try to get the method from the QuerySet instead.

With the QuerySetManager, we can define a QuerySet to use in the Ball model.

    class Ball(models.Model):
    	objects = QuerySetManager()
    	class QuerySet(QuerySet):
    		def get_exploded(self):
    			return self.filter(exploded=True)

Now we can use our custom method in whichever order we want.

    Ball.objects.get_exploded() # called on the ManagerBall.objects.filter(size=4).get_exploded().order_by('created_at') # called on a QuerySet

