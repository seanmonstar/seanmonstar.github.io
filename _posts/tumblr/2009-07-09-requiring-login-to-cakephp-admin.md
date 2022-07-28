---
layout: post
title: Requiring Login to CakePHP Admin
date: '2009-07-09T09:29:00-07:00'
tags:
- php
- cakephp
- bestof
tumblr_url: https://seanmonstar.com/post/707166936/requiring-login-to-cakephp-admin
---
For some of my freelance clients, I have provided a home-brewed CMS, built using CakePHP, since I’d already used Cake for the rest of their web-site. I wanted to create a couple users, and an interface to be able to create pages that made up the navigation and content.

Building an admin area for this functionality, the first thing I did was turn on [admin routing](http://book.cakephp.org/view/46/Routes-Configuration#Prefix-Routing-544).

Since we’re logging in Users, we build a `User` model and `UsersController`. In the User model, write whatever extra you need, but the basics is just a validation function, to match username with password when logging in.

    function validateLogin($data) {    
    	$user = $this->find(array(        
    			'username' => $data['username'],        
    			'password' => md5($data['password'])        
    		), array('id', 'username'));        
    	return !empty($user) ? $user['User'] : false;      
    }

You can add any additional validation rules in here, and you might want to use a [better hashing use than simpy straight md5](http://mcarthurgfx.com/blog/article/a-basic-lesson-in-password-hashing). Anyhow, this function will be called from the `UsersController`. So let’s dive into there.

#### UsersController

Throw together a simple login function. I’ll let your imagination put together the view, that’s just some simple html form stuff. Check for post data, validate with the above function, write the User object to the session and redirect to the admin area.

And conversely, logging out is simply destroying the User object from the session and redirecting out of the admin area. Simple stuff.

To help _enforce_logging in though, we pay attention to the filter event of controllers, and check the session first. The [process of a controller](http://book.cakephp.org/view/60/Callbacks) responding to the router is _beforeFilter -\> **action** -\> beforeRender -\> render -\> afterFilter_. You can make sure things happen before it calls any action, by writing a beforeFilter method. You could use this method to check for things, and if it’s not up to snuff, redirect to a different page. That’s what we’re going to do.

So, in the UsersController, we want to make sure that a user can’t do anything pertaining to users until they login.

    function beforeFilter() {    
    	if($this->action != 'login' && $this->action != 'logout') {        
    		if($this->Session->check('User') == false) {            
    			$this->redirect('login');           
    			$this->Session->setFlash('The URL you\\'ve followed requires you login.');        
    		}    
    	}
    }

#### Admin Requires Login

The last step is ensuring that any time a user wants to access the admin part of our app, they must be a logged in user, or they the boot. This involves using the same event as above, but in the `AppController`. The `AppController` lets us write something that should happen in every controller, because by default we should be extending `AppController`.

However, we don’t want it to **always** be forcing logins. Visitors who access public areas shouldn’t have to login. But if they want to access the admin areas, **that’s** when we want to force them.

With our admin routing turned on, any time the Router wants to invoke a method because of the `Routing.admin` configuration, it will add to the `params` of the controller `'admin' = 1`. Thus, we can easily make our filter only care if admin is set.

    class AppController extends Controller {     
    	function beforeFilter() {
            if ($this->params['admin']) {
                if($this->Session->check('User') == false) {
                    $this->flash("The URL you\'ve followed requires you login.",'/login',2);
                }
                $this->layout = 'admin';
            }
        } 
    }

#### Admin Galore

Now, any page we want to setup as requiring admin access, like in my CMS, allowing admins to edit pages, we create functions prepended with value in your config for `Routing.admin`. By default, that’s `admin`.

This simple addition now requires you be logged in (with a user made either manually by myself or using a registration form you can build into the `UsersController`) in order to see the list of pages.

    class PagesController extends AppController {    
    	function admin_index() {        
    		//setting the layout could be done in        
    		//the AppController beforeFilter, so all        
    		//admin pages use the admin layout instead.        
    		$this->layout = 'admin';                
    		$this->set('pages',$this->Page->getNav());
    	}   
    }

The default view for this follows the same kind of automagic rules for CakePHP: it’ll be `app/views/pages/admin_index.ctp`.

