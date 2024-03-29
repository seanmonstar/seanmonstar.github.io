---
layout: post
title: 'Podcast: Rustacean Station, hyper 1.0, and independent maintainership'
tags:
- rust
- hyper
- podcast
- open-source
---

I was recently a [guest on the Rustacean Station podcast][podcast]. It was nice to catch up after a couple years since [my last appearance](https://seanmonstar.com/blog/podcast-hyper-on-the-rustacean-station/) on the show. We spoke about hyper, how and why it became v1.0, becoming an independent maintainer, future work, and more.

I thought I'd grab a few fun and interesting quotes, with their timestamps:

### Sponsoring is not the Mafia (11:48)

> The lowest engagement is [sponsorship][sponsor] where it's like, what am I getting from your sponsorship? If you turn it around and think about, well, a lot of times open source is like this common good that everyone kind of benefits from and I don't want to pay for it because someone else is going to pay for it, right? It's kind of like this Russian roulette of like, well, if no one pays for it, then eventually that thing is going to go away. So sometimes when I talk to the companies, I just ask them, hey, how much are you using this stuff? You're using it a lot. Cool. How annoying or how much work would it be to have to maintain it yourself if I were to just disappear? Oh, then we'd have to put a whole engineer on that or something like that. Okay. So if you think of it as a business risk mitigation to sponsor, then I can keep doing it. And then it becomes like, okay, well, we would pay a full-time engineer to do this all year. We can pay a way smaller amount to just have the sanity check that this is not going away. And so what do they get? They get a business risk mitigation.

> _You'll have to excuse me, but I've been watching a lot of Sopranos. And so when you said, how bad would it be if this thing happened to go away? It sounded a little bit like a tactic they might use to secure the contract._

> (laughs) Yeah, it's not like an insurance racket.

### Access to the maintainer / advising (12:50)

> There's [more things that I can offer][sponsor] and I do offer. Another one is people want to be able to ask, hey, we're using your stuff and we want to know are we using it correctly or, hey, we've been trying to use it and we have this problem and we don't understand why. Could you take a look?
> If it's a private project, people aren't going to post their source code into a public issue. I'm not going to have the time to go and take a look unless it's like, hey, let's get a retainer and I can now sign an NDA. I'm not going to steal their code, all that legal stuff.
> It's like getting an [advisor][sponsor], a reviewer, and then I can then take that knowledge and go back and be like, okay, so I now have knowledge that like company so-and-so is using this in a way that I didn't expect. How can I make it better for them? So like, both sides benefit, but it's only possible if I set up contracts and everyone's legally happy.

### Breaking people, not kneecaps (30:55)

> If something would be a breaking change, then we label it with a breaking change. And it's not something we can do right now. We can close it or postpone it or something. And then, you know, maybe three years from now, go and take a look at all the issues that are labeled that and say like, yeah, you know what, this would be worth breaking for. But at the same time, I'd love to not have to break people, even though, you know, the promise is only for three years. I'd love to not have to.

> _Did you just say break people? We did go back to the mafia reference, right? But no relation, right?_

> Exactly. Yeah. No breaking kneecaps.

### Client middleware (32:12)

> The other major thing is improving middleware. There's all this stuff in [tower][], and it's great. Like there's [Axum][]. You can use it to make really powerful servers. But the point of this middleware was actually that you could use it both ways. You could use it for servers, but you could also use it for clients. And that doesn't work as well.
>
> The most popular thing to use for clients is [reqwest][]. And it doesn't fit in to tower middleware. It has a bunch of extra stuff like redirects and some other things that it does. Various kinds of timeouts, which would be great middleware, but it's currently all just wrapped up in reqwest.
>
> And then also, if you even just look at the middleware in the first place, it's like opaque. You have to understand how to do service discovery to then use load balancing. You have to understand how to build a retry policy before you can fit in retries and understand what is a retry budget.
>
> It's complicated. And it'd be nice if people could just get, hey, this is better for 95% of use cases. And if you really want to go and tweak your retry policy, sure you can. There's the escape hatch over there.
>
>  But that's my other thing to work on. Make reqwest and like Tower's client middleware merge together a whole lot nicer so you can have [much better stacked clients][reqwest-next].

### Retry storms and budgets (33:50)

> Since there wasn't a really easy plug-in, then people implement retries themselves. And that includes making themselves vulnerable to retry storms and just not doing it in a safe way. And the thing is that we have this middleware and things that would protect you from that, but they're just complicated enough that people are like, ah, I can just retry in a loop.
>
> But then you smash the server once things start falling apart. And it'd be so much better if you just add in a retry layer. It's going to do things wisely. And maybe you say, you know what, on this URL, never retry it, but otherwise do the default thing.
>
> It'd be so much nicer if you didn't have to understand how bad retries can go.
>
> If you just do a simple counter, then you might have counters at various layers. And maybe your counters aren't shared. And now even though you're trying to be nice to the server, you're still retrying on thousands of times, whereas a shared [budget][] or something would notice after a few retries, hey, the server's overloaded. Let's stop pounding it. Because as long as we keep trying, it's never going to get back up. That's one of the problems with retry storms.
> The [linkerd blog](https://linkerd.io/2019/02/22/how-we-designed-retries-in-linkerd-2-2/#using-retry-budgets) had a good post on how you can fix that using Tower middleware. But I'd prefer it if people didn't have to read that to use it.

### And plenty of other topics

Go [have a listen][podcast], I hope you find it informative!

[podcast]: https://rustacean-station.org/episode/sean-mcarthur/
[sponsor]: https://seanmonstar.com/sponsor
[tower]: https://crates.io/crates/tower
[Axum]: https://crates.io/crates/axum
[reqwest]: https://crates.io/crates/reqwest
[budget]: https://docs.rs/tower/latest/tower/retry/budget/struct.Budget.html
[reqwest-next]: https://seanmonstar.com/blog/reqwest-v012

