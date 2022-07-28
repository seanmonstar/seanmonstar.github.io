---
layout: post
title: '"Send To" is Easy in Android'
date: '2010-08-19T14:46:02-07:00'
tags:
- android
tumblr_url: https://seanmonstar.com/post/978986938/send-to-is-easy-in-android
---
Yesterday, I read a couple articles in my reader that, while unrelated, both touched on how irritating it is that applications in iOS can’t talk to each other.

[Shawn Blanc](http://shawnblanc.net/2010/08/ttttask/):

> A services menu for iOS. If this were a reality, people with the Things app installed on their iPhone and/or iPad could have a “Send to Things” service available within all their other apps. Thereby allowing them to send tasks into Things on their iPhone directly from Twitter, Reeder, Mail, Mobile Safari, and more.

[Khoi Vinh](http://www.subtraction.com/2010/08/18/ipad-gripe-session):

> I want an editable dictionary that can supplant iOS’ built-in auto-correct dictionary. I want Text Expander available to every app, and 1Password and Instapaper too, while we’re at it. I’m sure many readers out there have their own wish lists of similar enhancements, but we’ll probably never get any of them so I may as well stop here.

I’ve seen this myself when I’ve used friends’ iPhones, and using my wife’s iPod Touch. I’ve seen how people mention needing an API so that other applications can “send” data to them. There’s also some projects on Github that allow iOS developers to incorporate sending to Twitter, Facebook, or whatever in their own application.

This is something I don’t miss at all. The way Android handles this lets every application you download interact with any other application you have. Something like [Instapaper](http://instapaper.com), if it had an official Android application, could easily just define a “Send To” behavior, and not worry about making updates to allow sending to email, and then Twitter, and then Buzz, and then Google.Me, etc.

It requires a little bit of work for both applications to talk to each other, but it is indeed **little**. The sending application just needs to dispatch an `Intent` to the Android OS.

    Intent sendIntent = new Intent(Intent.ACTION_SEND);
    sendIntent.setType("plain/text");
    sendIntent.putExtra(Intent.EXTRA_TEXT, myText);
    startActivity(Intent.createChooser(sendIntent, "Send To:"));

The receiving application only needs to tell Android what Intents it cares about. Again, really easy. In the application manifest, you just setup an Intent filter.

    <intent-filter>
        <action android:name="android.intent.action.SEND" />
        <category android:name="android.intent.category.DEFAULT" />
        <data android:mimeType="text/plain" />
    </intent-filter>

This way, whenever any application wants to “send” plain text data (like a link) to another application, it just needs a vague send listener. Then, the sending application will prompt the user with a nice menu to send the data to.

![send to menu](https://64.media.tumblr.com/tumblr_l7f5o5tHYV1qzek7l.jpg)

Hopefully for iPhone users, Apple will come up with something similar in iOS 5.

