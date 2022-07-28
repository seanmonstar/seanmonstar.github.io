---
layout: post
title: Preparing SQL Queries
date: '2008-07-02T10:35:00-07:00'
tags:
- php
- sql
tumblr_url: https://seanmonstar.com/post/706950211/preparing-sql-queries
---
With web applications becoming more prevalent, and new developers showing up to fill the demand, security for web applications is increasingly important. Some applications have very special, very private data that should be only accessible to the specific user. Other, less “important” applications that don’t collect a lot of personal data, still have a need for security. With simple _SQL Injection_, your up and coming social media application could have it’s whole database wiped, just for kicks.

Thankfully, there’s a rather simple procedure to prevent the majority of malicious code injections by _preparing your SQL queries_ before sending them into your database. Let’s take a look at how to do so with _CodeIgniter_, but before that, how to do so using only the _PHP’s native PDO interface_.

##### SQL Injections

What is an SQL Injection? It’s helpful to know what this is so we can better understand what we’re defending against. One example is retrieving all user information:

    SELECT \* FROM users WHERE name = '.$userName.';

In the form field where we supply $userName, if we type _x’ OR ‘1′=’1_, then the query gets turned into:

    SELECT \* FROM users WHERE name = 'x' OR '1'='1';

Name may never equal x, but using _OR_, with an expression that is always true (1 = 1), we just recieved every row in the table.

One other example shows how instead of stealing data, a hacker could blow away your database. Typing _x’;DROP TABLE users; SELECT \* FROM users WHERE name=’x_

    SELECT \* FROM users WHERE name = 'x';DROP TABLE users; SELECT \* FROM users WHRE name='x';

With a little guessing that the table might be called users, we just set a 3-part chained query. The first returns nothing, cause no name is x, the second deletes all rows from users, and the third returns nothing, cause there’s no more users, but deals with the ending single quote (’).

Yikes! Just a little guess work, and our literally composed query can be easily manipulated to destroy an application or leak information. Here’s a [walkthrough case study of an anonomized injection](http://www.unixwiz.net/techtips/sql-injection.html) to see how a hacker might compromise your application. Now on to protecting ourselves from malicious injection.

#### Native PDO Interface

##### The Model

Let’s make a very basic Model class that will interact with our database. Note, this doesn’t include connecting to the database. I assume you know how to do so.

    \<?php class Model extends PDO { public static function query($query, $args) { $statement = $this-\>prepare($query); foreach($args as $num =\> $value) { $argNum = $num + 1; if(is\_null($value)) $statement-\>bindValue($argNum, $value, PDO::PARAM\_NULL); else if(is\_bool($value)) $statement-\>bindValue($argNum, $value, PDO::PARAM\_BOOL); else if(is\_int($value)) $statement-\>bindValue($argNum, $value, PDO::PARAM\_INT); else if(is\_string($value)) $statement-\>bindValue($argNum, $value, PDO::PARAM\_STR); else $statement-\>bindValue($argNum, $value); } $statement-\>execute(); return $statement-\>fetchAll(); } }?\>

Cool beans. Now we can structure a query in our controller, and remembering to include this class at the top, we can make a safe query:

    \<?php include('Model.class.php'); $id = 1; $result = Model::query("SELECT \* FROM table WHERE id=?",array($id);?\>

What we did here was contructed the query, but left out the values and substituted in question marks. We then passed the values as a seperate parameter. So what happens?

##### PDO::prepare() and PDO::bindValue()

First, your query is prepared using the prepare() function extended from PDO. This prepares your statement to have the values passed into it wherever there were question marks.

Next, for each value, the data type is checked, and then bindValue() from PDO is called, and checks that it is the proper data type. At this time, it will also place the proper single quotes, insuring our quotes aren’t manipulated.

Finally, PDO::execute() sends the bound query, and PDO::fetchAll() returns the rows of data from the query. You can read up more about the [PDO interface at php.net](http://php.net/).

#### Soul Binding with CodeIgniter

I personally prefer to [develop](http://../contact) using CodeIgniter, and the above functionality is already written for me. Let’s see how to very simply accomplish the same in this [wonderful framework](http://codeigniter.com/).

    $this-\>db-\>query("SELECT \* FROM table WHERE status = ? AND author = ?", array('Published', 'Sean'));

Ah, so very simple. Well, just as simple as above. But this is the CodeIgniter way.

##### Other Tips

A few other things that will _help prevent guesswork on your database_:

1. Make your column names with prefixs, or a personal touch to keep them out of the ordinary.
2. Same applies to your tables, give them specialized names like “sean\_blog\_users” instead of simply “users”.

<small>Employing these techniques will by no means make your application hacker-proof. There’s no such thing. Anyone can break security as long as they’re determined enough.</small>

