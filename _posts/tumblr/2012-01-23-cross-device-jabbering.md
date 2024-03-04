---
layout: post
title: Cross Device Jabbering
date: '2012-01-23T15:17:42-05:00'
tags:
- android
- ios
- jabber
- chat
- im
- communication
tumblr_url: https://seanmonstar.com/post/16361991702/cross-device-jabbering
---
People love to talk to each other. Especially when they’re friends. People do so every day across SMS, Facebook, and GTalk. It would be better for everyone if companies worked together to let everyone communicate with their friends in an easier fashion. It’d be easier than you might think.

GTalk comes as part of Google’s suite of apps by default on Android phones, so everyone who has one has an account. Plus, everyone who has a GMail account, is also signed up to use GTalk. Google+ Chat recently uses GTalk in the webapp, but the mobile version is some other service that can’t chat with people on a desktop computer.

iPhones, iPod Touches, and iPads sold now come with iMessage, which intelligently blends SMS with a chat-like service. The way it [manages to decide which service to use](http://apple.stackexchange.com/a/27317) is the model that all these companies should use for what I describe below.

Even if your friends don’t have smartphones, many have a Facebook account, and use it all day long. Facebook released an app, Messenger, that does similar things as iMessage, where it tries to use its Chat service to talk to your friends before reverting to SMS.

GTalk and Facebook Chat already use the Jabber protocol (or [XMPP](http://en.wikipedia.org/wiki/Extensible_Messaging_and_Presence_Protocol)) to communicate. There’s reasons to believe that iMessage could be close to [supporting the Jabber protocol](http://www.theverge.com/2011/11/17/2569612/ios-5-code-aim-jabber-chat). iMessage and Facebook Messenger already intelligently pick which service to use. These 3 companies could work individually to allow their application to try to guess, based on contact information you already have of the user, if they can be reached through the Jabber protocol on one of the other services. This would cut down tremendously on the need for SMS, as well as alleviate the need for a user to have to remember the best way to contact one of their friends. [The computers should figure this stuff out for us!](http://seanmonstar.com/blog/universal-communicator/)

I understand the desire for lock-in[^1], but at this point, each service is so big, it would be better to improve communication between them, and compete on other features. Each company needs to admit that they aren’t going to have _all_ the users, and so it would actually make their service better if it included our friends on other services.



[^1]: Is there anything here with saying Facebook Chat is using it’s social network advantage to require people to only contact Facebook friends through Chat? Sounds ridiculous, right? It also sounds a lot like the complaint over [Search Plus Your World](http://seanmonstar.com/blog/antitrust/).

