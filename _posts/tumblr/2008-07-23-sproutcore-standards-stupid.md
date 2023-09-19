---
layout: post
title: SproutCore - Standards Stupid?
date: '2008-07-23T13:39:00-04:00'
tags:
- javascript
- standards
- opinion
tumblr_url: https://seanmonstar.com/post/707010197/sproutcore-standards-stupid
---
Steve Webster recently wrote an article about how [horribly standards-stupid](http://dynamicflash.com/2008/07/the-problem-with-sproutcore/) SproutCore ([the Javascript framework](http://sproutcore.com/) Apple used to make [MobileMe](http://me.com/)) is. He _kind of_ has the right mind-set, in that _Javascript should be a progressive enhancement to web-sites_, and they should still function properly without it.

I agree completely. I feel all AJAX commands that you include in your web-site should do the proper PHP call if Javascript is disabled. Tabs should only contain non-essential information, and the most important should be on the first one, in case it’s not possible to switch between them. But are there _exceptions to the rule_?

##### Web Applications - Exceptions?

Now, granted, not all web applications are exceptions. If your application is to be used by the commons, and you’re concerned about people you could leave off, then certainly, make your application standards-smart. Google is a perfect example of this. But must everyone?

#### SproutCore

##### Javascript Scarcity?

SproutCore applications are rich desktop-like programs with lots of functionality in them. The majority of the application is written in Javascript. So of course, people without Javascript aren’t going to be able to use it. But how many people is that? All computer browsers come with Javascript. Mobile browsers suck, but would you expect someone to use a rich application from their crappy mobile device? Excepting the iPhone, which does have Javascript in its broswer.

> They also seem to have missed the fact that the percentage of users unable to see Flash content is significantly lower than those fumbling around the Internet without JavaScript disabled. According to Adobe’s statistics content published for Flash Player 9 is viewable by an average of 97.4% of web users across all markets. That compares very favourably with the 95% of users who have JavaScript enabled according to w3schools.

**There’s a problem with this statement of statistics**. Firstly, 2% is not a significant number. But worse, while there is a percentage of people browsing the Internet with Javascript disabled, _they all have the easy method of going into options and re-enabling it_. And that shouldn’t be too hard, since they had to know enough to disable it the first time. But all those people without Flash have no easy option. You must provide a link to download the proper Flash player, which can be a mess who isn’t so computer-savvy (they exist, and I’m the one the come asking to fix things).

##### An Enhancement?

Here’s why _SproutCore tends to be an exception_: The application is written with Javascript in mind. It’s written to use Javascript. It’s saying to the users: **Hey, you want to use Application Awesome? Cool. It runs in Javascript, so just enable that baby and awe your mind.**

If someone doesn’t have Javascript enabled, telling them to enable it to use your Javascript Application is not a sin. It’s much more of an ordeal to write the same functionality in a bunch of dynamic pages, incase the user doesn’t want to turn on Javascript. _SproutCore isn’t used to make enhancements to your web application_. Ajax-y form submittals are enhancements. It’s OK to make the User be semi-proactive by clicking Enable Javascript. No one complains that web-sites require a Flash plug-in, or that Java applets require a Java Virtual Machine.

<small>Disclaimer: Most likely, you’re application has Javascript as an <strong>enhancement</strong>, if it has Javascript at all. Do not use this article as a reason to shoot standards in the face. This is directed at those rare applications that are built in Javascript.</small>

