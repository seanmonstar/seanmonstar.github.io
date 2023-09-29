---
layout: post
title: Gmail Bridge for Persona
date: '2013-08-08T18:22:00-04:00'
tags:
- mozilla
- persona
- identity
- gmail
- planet
- bestof
tumblr_url: https://seanmonstar.com/post/57737181854/gmail-bridge-for-persona
---
Since [shifting to the Identity team](http://seanmonstar.com/blog/moved-to-identity/) last year, I’ve been working hard on making Persona a true solution to the login problem of the web. As I said then:

> If we do our job right, eventually when my friends ask me what I do, I can say: I helped make it so you no longer need to use passwords everywhere. I helped make your online identity more secure. I helped make signing into the Internet awesomer.

We’re getting closer.

### What is the Gmail Bridge?

Today, we’re announcing to the world that our [Gmail Identity Bridge is online](http://identity.mozilla.com/post/57712756801/persona-makes-signing-in-easy-for-gmail-users). Excuse me. What? No, I’m fine. It’s alright, it’s actually quite simple.

The way [Persona normally works](http://lloyd.io/how-browserid-works), after checking to see if your email provider natively supports the protocol, is that Persona will fallback to what we call a secondary provider. This is the point where most users end up creating a password for Persona, and then going to their email to verify to us that they really own their email address. If the email provider _did_ support the protocol, they would get sent over to them to authenticate, and we’d step out of the way.

So, we made an [Identity Bridge](http://identity.mozilla.com/post/56526022621/what-is-an-identity-bridge) that we host, and uses Google’s OpenID endpoint to verify the user. The experience is pretty much exactly what it should feel like if there was native support from Google.

### Why this matters

With both Gmail and Yahoo bridges online, _over half of all users_ are just a couple clicks away from logging in with Persona.

So how does this affect you? If you have a website that has user accounts, you can switch to using [Persona as your authentication system](http://davidwalsh.name/introduction-persona). In most cases, it should be a **better** experience for your users, and easier for you.

If you don’t have a website, you can still help. Find a website you log in to frequently, and ask them to implement Persona. Tell them about this new bridging. Push for the change.

Soon, everyone will notice: **we made signing into the Internet awesomer.**

