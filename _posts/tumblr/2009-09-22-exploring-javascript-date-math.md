---
layout: post
title: Exploring Javascript Date Math
date: '2009-09-22T10:25:00-04:00'
tags:
- javascript
tumblr_url: https://seanmonstar.com/post/708726028/exploring-javascript-date-math
---
Dates are a peculiar type of data that we have to work with. In some sense, they aren’t a number, but instead are a combination of month, day, and year. But at the same time, in most programming languages, they are fundamentally a number: the number of seconds since the [Unix epoch](http://en.wikipedia.org/wiki/Unix_epoch).

In our heads, it’s quite easy to do Date math. Moving forward a week requires just adding 7 days. How would you do that with a Javascript Date? Programming languages usually try to make working with Dates as logical as it is to do in our heads. So I a little surprised at an irregularity in using Dates in my math.

Though to be fair, it seems I could blame it on Javascript’s decision to make the addition operator and concatenation operator the same symbol. I only did some quick, sloppy tests in Firefox 3.5 and Chrome 3.

#### Adding to Dates

I first wanted to move a date forward a week, so I thought that perhaps I could add 7 days worth of milliseconds to the date to get next week. Go ahead and try it:

    new Date() + (1000 * 60 * 60 * 24 * 7)

The value is a string: “Fri Sep 18 2009 16:37:01 GMT-0700 (Pacific Daylight Time)604800000”. Yup, there it is! Our week’s worth of milliseconds, attached on the end of a Date string. It would seem that Javascript decides that the Date should be evaluated as a string instead of a number at that point.

##### How To Add

In order to add to Dates, I had to create a new Date, using the value returned from [getTime](https://developer.mozilla.org/en/Core_JavaScript_1.5_Reference/Objects/Date/getTime)&nbsp;and adding milliseconds to it, and using that new (humongous) number in the constructor.

    var nextWeek = new Date(today.getTime() + 1000 * 60 * 60 * 24 * 7);

This is because the Date’s toString method is being evaluated instead of it’s valueOf. We can also overcome that with an additional plus sign.

    +(new Date()) + (1000 \* 60 \* 60 \* 24 \* 7)

Also to keep in mind here, is that unlike in, say, PHP where you provide seconds to the [date](http://php.net/date) function, in Javascript, you must provide _milliseconds_, hence the extra 1000 in there.

#### Subtracting from Dates

Now, until I remembered that + means to different operations in Javascript, I was rather shocked that I could easily subtract from Dates, but not add.

    new Date() - (1000 * 60 * 60 * 24 * 7)

Result is a number: 1252714884521. This does the proper math, but not everything I had hoped and dreamed for. No matter, receiving a number is certainly a result I could expect, as opposed to the string joining from addition.

