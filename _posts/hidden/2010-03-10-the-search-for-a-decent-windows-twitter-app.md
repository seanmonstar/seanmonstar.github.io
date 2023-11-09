---
hidden: true
layout: post
title: The Search for a Decent Windows Twitter App
date: '2010-03-10T18:19:00-05:00'
tags:
- twitter
- opinion
tumblr_url: https://seanmonstar.com/post/709000980/the-search-for-a-decent-windows-twitter-app
---
I kinda like [Twitter](http://twitter.com/seanmonstar). It’s a fun place to leave [occasional comments](http://mcarthurgfx.com/blog/article/i-m-a-twitter-monstar). It’s also a great way to find links to interesting information I would normally miss from my feeds. But being a Windows user, I have yet to find **that** Twitter application that is a joy to use.

I’ve tried out a couple applications, and found each one lacking:

- [Tweetdeck](http://www.tweetdeck.com/) 
- [DestroyTwitter](https://destroytwitter.com/) 
- [Seesmic](http://seesmic.com/seesmic_desktop/windows/)

There might be a decent application out there that I haven’t found, but these are the most popular ones available to Windows. It almost feels like people only want to make cool stuff for Macs.

#### Tweetdeck

Although it’s hugely popular by many, I can’t help but get annoyed at parts of Tweetdeck. To be fair, it’s the closest I have found that meets my needs, but that’s not good enough for me. It can easily handle auto-completing of replies, inserting photos and shortening links. It’s built with the idea of lists in mind, which I particularly like. I’ve currently got a list about MooTools and a list about games. Unfortunately, I have no option of telling the main list to ignore tweets from anyone in a separate list. I don’t want my main list filled with tweets about stuff I’ve already put into it’s own list. I just want a list of “everyone else”, but without having to create such a list and add every new person I follow to it.

<figure class="tmblr-full" data-orig-height="332" data-orig-width="500"><img src="https://64.media.tumblr.com/cc111d0a76037e89880d0ad501fe82d1/74c50fe3067f380f-78/s540x810/240657556d3654dafab6f23f87ded68852f37da2.png" data-orig-height="332" data-orig-width="500"></figure>

I also have gripes about it’s UI. Marking Tweets as “Read” requires _clicking individually on this tiny, 5px wide circle_. And then after doing that, I must click at the bottom of the list of a tiny icon to “Clear Read”. And I often accidentally click on “Clear All”, since they’re right next to each other, and they’re so small it takes to long to visually register what each icon is.

The notifications system is buggy. Sometimes it hides the summary behind a window, sometimes not. Clicking the notification will bring up Tweetdeck. But sometimes, the notification won’t dismiss itself. Ever. You can hide Tweetdeck again and have a frozen notification sitting in the corner of your screen. The only way to dismiss it is to click it again. The usual behavior is that it should dismiss regardless after a few seconds.

And it’s an AIR application. Now, granted, AIR is pretty cool. It lets front-end developers create apps for the desktop. But given a choice, why would I want to run something in a Flash player all the time? **It chews on my memory like a puppy that is teething**. It lacks plenty of things that I’m used to having in a Windows app. And it’s integration is buggy (see above about notifications). Next.

#### DestroyTwitter

While it’s also an AIR app, which means I have similar complaints in that regard, it’s actually less buggy than Tweetdeck. Notifications don’t stick. I still miss my normal Windows feel though.

It has a much nicer way of handling Mark as Read: just check the option in settings to Mark as Read on rollover. And overall, the UI feels much nicer to use than Tweetdeck. You don’t have to hover over the avatar to find your common actions like replying or retweeting.

<figure class="tmblr-full" data-orig-height="458" data-orig-width="316"><img src="https://64.media.tumblr.com/417989f692dcd6ae4a1ac989bd97b06f/74c50fe3067f380f-d6/s540x810/e6499c8bb065d5766dbbad5f444f7e16bc9f9c9a.png" data-orig-height="458" data-orig-width="316"></figure>

However, it doesn’t support lists at all. None. Just a single stream of tweets. No thanks.

#### Seesmic

A native Windows client, and it doesn’t have the Windows XP look either. It generally looks sweet, feels rather Windows-y. I like that it has useful context menus when I right click on things. It’s design is decent excellent. But **it’s severally lacking in features**. Severely.

<figure class="tmblr-full" data-orig-height="358" data-orig-width="500"><img src="https://64.media.tumblr.com/7906cc02d0b6b55585954c6c04f26e7c/74c50fe3067f380f-a9/s540x810/33ba9900fe2fea90a79b403a2059571d78638b85.jpg" data-orig-height="358" data-orig-width="500"></figure>

It can’t load images from common sources like yfrog, it just launches the browser. While it doesn’t have automatic URL shortening, it does have an Insert Link ability. But my attempts to use it show that it handles errors poorly:

    \<bitly\> \<errorCode\>0\</errorCode\> \<errorMessage\>\</errorMessage\> \<results\> \<nodeKeyVal\> \<errorCode\>101\</errorCode\> \<errorMessage\>Unknown error\</errorMessage\> \<nodeKey\>\<![CDATA[http://google.com]]\>\</nodeKey\> \<statusCode\>ERROR\</statusCode\> \</nodeKeyVal\> \</results\> \<statusCode\>OK\</statusCode\> \</bitly\>

<small>Seesmic just dumped that straight into my message. And then it kindly warns me that I’m 282 characters over the limit.</small>

While it supports lists, it has the same issues as Tweetdeck in that regard. But, clearing the timelines can also be wonky. I’ve clear my MooTools list timeline, and then added a new person to the list, and it just went out and refilled the entire timeline. Er, thanks.

**And it’s bugtastic**. Maximizing the window hides away the normal minimize/close buttons. You have to just click in the area and hope you guessed correctly. Even with notifications turned on, I have yet to see a single notification. It regularly errors when trying to get more tweets, and leaves me without updates for long periods of time.

#### web

Ultimately, at home, it’s just easier to use the web client. None of the clients I have found are good enough to do light browsing with. I do most of my reading at work, so when I bother to log on at home, I just want the latest part that I haven’t read yet. **None of the applications I’ve tried seemed to have syncing**. Loading one up at home would just dump the whole days worth of tweets into my timeline, and that’s useless to me.

Since the web application defaults to showing me the latest tweets, that’s usually all the ones I want to see anyways. Replying and retweeting are easy enough, although it would be great if it automatically shortened URLs for me.

#### “That” Twitter App

The application I’m looking for is a native client to Windows, doesn’t look like it came from the year 2000 in terms of UI, and has many of the features you’d hope to have in a Twitter client: _URL shortening, image loading, notifications, and lists_. I’d love the option to remove all listed people from my main timeline, but whatever. **If it has those features, looks pretty (and is intuitive), and isn’t the buggiest piece of code in the world, I’d like to use it**. But it really seems like so many developers only like making things for Macs now. There’s got to be developers who still work on Windows… or maybe I should just break out the C#-fu.

