---
layout: post
title: Hacking To Meet Deadlines
date: '2009-12-21T10:06:00-05:00'
tags:
- opinion
- php
- bestof
tumblr_url: https://seanmonstar.com/post/708885956/hacking-to-meet-deadlines
---
As a deadline approaches far faster than you can type, you’re required to write some quick-and-dirty code to fulfill those feature requests.

In case you don’t know what I’m talking about, this is when there happens to be a flaw in your program’s structure. It’s an architectural problem: you did properly build the system to elegantly behave in an expected manner. Sometimes, it’s a problem from bad planning at the start. In other cases, it comes from scope creep, where features get slipped into a system that previously was not going to have such features. [Gamasutra had a nice write-up](http://www.gamasutra.com/view/feature/4111/dirty_coding_tricks.php) about cases like this happening in the games industry. To put it as they did:

> Programmers are often methodical and precise beasts who do their utmost to keep their code clean and pretty. But when the chips are down, the perfectly-planned schedule is shot, and the game needs to ship, “getting it done” can win out over elegance.
> 
> In a case like this, a frazzled and overworked programmer is far more likely to ignore best practices, and hack in a less desirable solution to get the [code] out the door.

My favorite example in that article is about a game where in a certain level, some game object needed to be hidden. Instead of doing things the proper way, code was written along the lines of:

    if( level == 10 && object == 56 ){    
    	HideObject();
    }

#### I’m guilty of that

We know that happens far too often in our industry. Just the other night, I was guilty of doing exactly that.

As a developer for a [commercial CMS](http://www.blazonco.com), we provide basic e-commerce functionality. Recently, a client specifically needed coupon codes added to our getup. With a rather short deadline, and being busy as the year gets close to ending, I didn’t have much time to flesh out the design of this new functionality. Besides everything else we needed to do, this feature ended up with a 2 day implementation, with very little planning. Things ended up looking like this:

    foreach($items as $item) { if($item instanceof CouponItem) { //get discount } //do stuff with CartItems}

I just tied in where we were calculating the totals of all CartItems, and if one of the items was a CouponItem, do discount stuff instead. Another situation was in the view, when showing the cart. I had another loop through the cart items, in order to show them all. With CouponItems now apart of the Cart, I had to handle the different properties of a CouponItem there.

    <? foreach($items as $item) { ?>
    <td>
    	<? if($item instanceof CouponItem) { ?>
    	<!-- Coupon stuff -->
    	<? } else { ?>
    	<!-- normal stuff -->
    	<? } ?>
    </td>
    <? } ?>

One can’t help but feel dirty doing this. Just hodge-podging rules into place, that magically make everything all better.

#### Is it really that bad?

Some may be wondering, what’s the big deal? It works, doesn’t it? Why, yes! It works! And depending on your situation, that may be all that’s really necessary. After all, the point of software is to ship it. And it’s got to work. In the end, _how_it works on the inside has little value to the end user. That’s the same in my case. In the end, what really matters is that users can add products to a shopping cart and then checkout.

However, at the same time, since our product is something that I have to work on every day, writing software like this makes things more confusing, and hard to maintain. Come a few months from now, when we find a bug in the Cart Items somewhere, or when I need to add a new feature in there, that hacked in behavior is **unexpected behavior**. Unexpected behavior means its easy to break something when I modify code elsewhere. It also means that it will take more time to learn what the heck I was doing when I read it again in the future. _And I at least wrote that code_. The other developers have it worse.

Now that the deadline has passed, and it just works, I can spend the next couple days planning out a better, more elegant way of handling these coupon codes. My users won’t see the difference. But I will certainly notice it in a month or more.

