---
layout: post
title: Persona is dead, long live Persona
date: '2014-03-11T14:19:57-04:00'
tags:
- persona
- mozilla
- planet
- fxa
tumblr_url: https://seanmonstar.com/post/79278627673/persona-is-dead-long-live-persona
---
The [transition period](http://identity.mozilla.com/post/78873831485/transitioning-persona-to-community-ownership) was really tough for me. It felt like we were killing Persona. But more like tying a rope around it and dragging it behind us as we road tripped to Firefox OS Land. I first argued against this. Then, eventually I said let’s at least be humane, and take off the rope, and put a slug in its head. Like an Angel of Death. That didn’t happen either. The end result is one where Persona fights on.

Persona is [free open source software](https://github.com/mozilla/persona), and has built up a community who agree that decentralized authentication is needed o the Internet. I still think Persona is the best answer in that field, and the closest to becoming **the** answer. And it’s not going away. We’re asking that the Internet [help us make the Internet better](http://identity.mozilla.com/post/78873831485/transitioning-persona-to-community-ownership).

### Firefox Accounts

In the meantime I’ll be working on our [Firefox Accounts](https://wiki.mozilla.org/Identity/Firefox_Accounts) system, which understandably could not rely entirely on Persona[^1]. We need to keep Firefox competitive, since it’s what pays for us to do all the other awesomizing we do. Plus, as the Internet becomes more mobile and more multi-device, we need to make sure there is an alternative that puts users first. A goal of Firefox Accounts is to be pluggable, and to integrate with other services on the Web. Why should your OS demand you use their siloed services? If you want to use Box instead of iCloud, we want you to use it.

How does this affect Persona? We’re actually using browserid assertions within our account system, since it’s a solved problem that works well. We’ll need to work on a way to get all sorts of services working with your FxAccount, and it might include proliferating browserid assertions everywhere[^2]. As we learn, and grow the service so that millions of Firefox users have accounts, we can explore easing them into easily and automatically being Persona users. This solves part of the [chicken-egg problem](https://news.ycombinator.com/item?id=7364465) of Persona, by having millions of users ready to go.

I’d definitely rather this have ended up differently, but I can also think of far worse endings. The upside is, Persona still exists, and could take off more so with the help of Firefox. **Persona is dead, long live Persona!**



[^1]: Sync needs a “secret” to encrypt your data before it’s sent to our servers. The easiest solution for users is to provide us a password, and we’ll stretch that and make a secret out of it (so, we don’t actually know your password). Persona doesn’t give us passwords, so we can’t use it.

[^2]: Where “browserid” assertions are accepted, Persona support can also be found.

