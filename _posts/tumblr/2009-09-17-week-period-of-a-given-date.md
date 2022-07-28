---
layout: post
title: Week Period of a Given Date
date: '2009-09-17T08:00:00-07:00'
tags:
- php
tumblr_url: https://seanmonstar.com/post/708814441/week-period-of-a-given-date
---
As I work on an internal time tracker for our company, I needed to show all the TimeEntries for a specified week. To specify which week, it made sense to simply select 1 day in the week, since that’s the easiest default control in Flex. This let’s me get Sunday and Saturday, the start and end of the week, so I can build a query that grabs entries between those 2 dates.

#### In PHP

    $dayOfWeek = date('w', $date);
    $secondsInDay = 60 * 60 * 24;
    $sunday = date('Y-m-d', $date - ($dayOfWeek * $secondsInDay));
    $saturday = date('Y-m-d', $date + ((6 - $dayOfWeek) * $secondsInDay));
    $entries = TimeEntry::find(array('timestamp =<' => $saturday,'timestamp >=' => $sunday));

#### In English

First off, PHP’s [date](http://us2.php.net/manual/en/function.date.php) method can be given a parameter to return the numerical day of the week. So, if $date were today, we’d be storing 4 (Sunday is 0). Now, with just a little bit of math, we can subtract the number of days to get Sunday, and add the difference to get Saturday.

That last bit is a [data model](http://en.wikipedia.org/wiki/Modelviewcontroller) searching for all entries between those dates.

