---
layout: post
title: Overloading Objects in PHP
date: '2008-12-18T02:00:00-08:00'
tags:
- php
- bestof
tumblr_url: https://seanmonstar.com/post/708657446/overloading-objects-in-php
---
In PHP, objects are all dynamic. If you declare a variable for object after instantiation, it just throws it right in, no questions asked. Much friendlier than, say, Java, where you absolutely must define a variable prior to use or the JVM will smite you. PHP also lets you define extra or different instructions when using a previously unknown variable with magic functions.

#### \_\_get and \_\_set

Usually the native implementation of these functions is the desired result. But sometimes, adding some extra features into getting or setting a variable can really make things easier.

##### \_\_get

Let’s say we have a model called `Person`:

    class Person {    
    	public function __get($name) {        
    		$this->$name = Model::query('select * from people_table where name=? limit 1',array($name));        
    		return $this->$name;    
    	}   
    }

…and we try to access a variable that doesn’t exist:

    $obj = new Person;
    echo $obj->Sean['last_name'];

Since Sean isn’t a predefined value in `$obj`, it queries the database, grabs the row with name Sean, and returns it. And the echo statement proceeds to print my last name to the screen. I’ve also stored the result in the variable requested, so concurrent requests will get the stored variable and leave my database alone.

##### \_\_set

We could also try to do something in the reverse, by setting an unknown property.

    class Person {    
    	public function __set($name, $value) {        
    		$value []= $name;        
    		$this->$name = Model::query('insert into Person_table last_name=?, job=?, name=?',$value);    
    	}   
    }

Now this function doesn’t do a bunch of checking, and you’ll probably want do some. But since this is a simple example, I’m not. I’m just assuming that any unknown variable that I try to set on the model should be part of the database:

    $obj = new Person;
    $obj->Sean = array('last_name' => 'McArthur', 'job' => 'Developer');

There are some actual real good uses for this overloading; the ones I’ve shown are simplistic and possible a little too extreme. But now with the understanding of these magic functions, hopefully you can put them to good use.

#### \_\_call (and \_\_callStatic)

The `__call` function helps us when we try to call a function that doesn’t belong to an object. Overloading this function is quite often used in API implementations. Let’s look at a small example:

    class PersonAPI {    
    	//query function    
    	//...        
    	public function __call($name, $args) {        
    		$this->query($name,$args);    
    	}   
    }

Assume we have a query function which makes a connection and tries to make a function call on a foreign API. Imagine `$person` is an object of our `PersonAPI`, and the follow two statements would then be identical:

    $person->query('getLists',array('token'=>'test'));
    $person->getLists(array('token'=>'test'));

This is very simple, one line solution. You could, of course, make it much more interesting than that.

##### \_\_callStatic

The same can done with static methods, so when we call a static method that doesn’t exist, instead of getting the error thrown in our faces, we could try to see if there’s something extra to do first.

    class PersonAPI {    
    	//static query function    
    	//...       
    	public function __callStatic($name, $args) {        
    		self::query($name,$args);    
    	}  
    }

#### Get Overloading

After knowing this, it’s pretty easy to fill in these functions for several classes you have. You could consider throwing common functionality into them, therefore getting a certain procedure on a simple get or set command.

