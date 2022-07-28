---
layout: post
title: A table that should exist in all projects with a database
date: '2010-12-09T11:00:00-08:00'
tags:
- sql
tumblr_url: https://seanmonstar.com/post/2156394463/a-table-that-should-exist-in-all-projects-with-a
---
[A table that should exist in all projects with a database](http://blog.cherouvim.com/a-table-that-should-exist-in-all-projects-with-a-database/)  

    create table schema\_version ( `when` timestamp not null default CURRENT\_TIMESTAMP, `key` varchar(256) not null, `extra` varchar(256), primary key (`key`) )

