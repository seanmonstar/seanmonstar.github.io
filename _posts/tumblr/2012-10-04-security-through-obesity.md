---
layout: post
title: Security through Obesity
date: '2012-10-04T16:03:40-04:00'
tags:
- security
- passwords
tumblr_url: https://seanmonstar.com/post/32887710004/security-through-obesity
---
[Security through Obesity](http://www.opine.me/a-better-way-to-store-password-hashes/)  

A really interesting way to store passwords. Short version: store the hashes in a table, with no foreign key what-so-ever, such that:

1. The owner of a given password is no longer explicitly discernible.
2. You can fill the `hashes` table with tons of false hashes, making it difficult to know which hashes to brute force.
