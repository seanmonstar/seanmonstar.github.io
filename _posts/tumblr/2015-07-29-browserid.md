---
layout: post
title: What’s the password?
date: '2015-07-29T09:25:29-07:00'
tags:
- persona
- identity
- browserid
- mozilla
- planet
tumblr_url: https://seanmonstar.com/post/125352745992/browserid
---
Exploring the wilds of the internets, I stumble upon a brand new site that allows me to turn cat images into ASCII art.

No way. Cats? In text form?! Text messages full of kitties!

This is amaze. How do I get started?

Says here, _“Just create an account.”_ Ok.

_“What’s your username?”_ seanmonstar.

_“Pick a p͏a̵ss–”_arrgghHHHa̵ąz͏z͝ef́w͟qa̛a̸s̕s̡;. **WHAT! NO!** What did you just call me?<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup> I will not!. I don’t care how amaze textual kitties might be.

* * *

Sorry, I’m fine now. It’s just… you know. It is downright [irresponsible](http://seanmonstar.com/2022/07/28/2014-03-25-your-password-is-insecure.html) at this point to require a user to enter a password to login to your site. It’s pretty easy to properly hash some passwords, but **DON’T DO IT!**. Instead, you should let a secure identity provider provide the user’s credentials.

Good idea! Er, which do you pick? There’s several, and each has its own peculiarities regarding its API. _Sigh_.

### Persona?

I had hoped [Persona](https://persona.org) would move us away from these dark ages, but it struggled to gain support. The user experience was disruptive.

Most often, users had to create a new Persona account. Oh hey, another password!

Additionally, they would need to go through email verification, which while it helps to make it secure, is another step that may cause a user to bail out. Even if they went through the whole process, web developers needed to properly use `navigator.id.watch()` to keep state. It was very easy to mess that up.

The popup would confuse users. We’ve been teaching users forever to distrust popups, but saying this one was okay.

### ++

At this point, most browsers have user account information already. Chrome has a Google account, Firefox has a [Firefox account](https://accounts.firefox.com), Safari has iCloud, and Edge has a Microsoft account. How about we just move websites to asking the User Agent for credentials, instead of the User directly?

This was what I originally assumed BrowserID would work, when I first heard about it. A user can sign into a browser, using whichever way that browser supports. The website (and thus web developer) isn’t required to care what account system the user wants to use. They just want to know “who are you” and “how can I be sure?”<sup id="fnref:2"><a href="#fn:2" class="footnote-ref" role="doc-noteref">2</a></sup> **The problem this solves now is passing and storing passwords.**

### `navigator.auth.get()`

A website could ask for credentials from the navigator, and the browser can show its own trusted UI asking the user if and which ID to share to the website. The API could return a `Promise<JWT>`, with the JWT being signed by the browser. There’s already standards in place for [verifying a signed JWT](https://tools.ietf.org/html/rfc7515), so the web developer can be confident that the user owns the data include in the token. An example usage:

    navigator.auth.get().then((token) => fetch('/verify', {
        method: 'POST',
        body: JSON.stringify(token)
    }))

Using a `Promise` and using the already-logged-in accounts of the browser, the website won’t need to reload. It therefore doesn’t need to store state, and doesn’t need the wonkiness of Persona’s `watch()`. This is simply an authentication requesting mechanism, so there’s no confusion about who manages the session: the website does.

Additionally, an HTML element could be used for sites that wish to support NoScript:

    <auth method="POST" action="/verify" label="Login" />

This could render like a `<div>`, and a website could style it to their spleen’s content. Another [pain point](https://groups.google.com/d/topic/mozilla.dev.identity/12PW2Z-YPps/discussion) of Persona solved.

### We can do this!

The Identity team at Mozilla is interested in exploring this, and being that the scope is low, working towards consensus and a standard is the goal, as opposed to Persona’s hope of adoption before standardization. **To a less-passwords web!**

* * *

1. 

“Password” should be a curse word. _Hey! Did you see that little password? Too busy texting to its passwording buddies, and almost hit me!_&nbsp;[↩︎](#fnref:1)

2. 

Federation of an account system should certainly be possible, but out of scope for this article. The points is that any browser maker can explore how to log into the browser, and pass a JWT to a website that includes a way to verify it. Firefox and others would then be free to explore de-centralized accounts and profiles, while web developers can happily log users in without evil passwords.&nbsp;[↩︎](#fnref:2)

