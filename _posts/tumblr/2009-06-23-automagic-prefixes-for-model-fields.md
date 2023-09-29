---
layout: post
title: Automagic Prefixes for Model Fields
date: '2009-06-23T12:40:00-04:00'
tags:
- php
- kohana
tumblr_url: https://seanmonstar.com/post/708669460/automagic-prefixes-for-model-fields
---
Say we have a player model, and every field in `players`table is prepended with _player\__. For example, `player_username`, `player_email`, etc.

I’m personally not used to this database design, but I know plenty of people use it. When I work on projects that have this, I’m not particularly found of having to write:

    $p = new Player;
    echo $p->player_username;

I’d rather ditch the prepended part in all my PHP code.

    echo $p->username;

#### Use Accessors

We can do this by [writing some \_\_get and \_\_set functions](http://seanmonstar.com/blog/overloading-objects-in-php/):

    public function __get($name) {    
    	$prepend = 'player_'.$name;    
    	if(isset($this->$prepend)) {        
    	return $this->$prepend;    
    	}    
    	return parent::__get($name);
    }
    
    public function __set($name, $value) {    
    	$prepend = 'player_'.$name;    
    	if(isset($this->$prepend)) {        
    		$this->$prepend = $value;    
    	} else {        
    		parent::__set($name, $value);        
    		//if no parent, you might want the default:        
    		//$this->$name = $value    
    	}
    }

Basically, [stated before](http://seanmonstar.com/blog/overloading-objects-in-php/), these get called when you try to access a property that doesn’t exist on the object. So when we try to access `username`, we check if `player_username` exists, and if so, return that value.

#### MY\_Model: Easily extendable

You could work this into a MY\_Model class that extends Model, and then make all your models extend MY\_Model. If you wanted to do this, I’d say make a property of MY\_Model called ‘prefix’, and use prefix in the accesors. Then, in each sub-class, all you need to do is define the prefix.

    class MY_Model extends Model {    
    	protected $prefix;    
    	public function __get($name) {        
    		$prepend = $this->prefix.$name;        
    		//...    
    	}   
    }

    class Player_Model extends MY_Model {    
    	protected $prefix = 'player_'; 
    }

