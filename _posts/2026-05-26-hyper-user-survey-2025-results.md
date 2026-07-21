---
layout: post
title: "hyper User Survey 2025 Results"
excerpt: "Analyzing the results from the hyper User Survey 2025."
date: '2026-05-26T09:02:00'
tags:
- rust
- hyper
- survey
- open-source
---
[hyper][] is the widely-used HTTP library for Rust. We ran the first [user survey for hyper in November 2025][survey-2025]. Here's the results and some analysis.[^burnout]

## Why did we do a survey?

We ran a survey to make sure we focus on the right things. It's part of being user-driven, working backwards.

The amount of work required to get _some_ data has an outsized return on your investment. When you know nothing, just a little bit of work means you now know _something_. And knowing what users need is a requirement to making an actually useful tool.

I keep [high-touch, high-context relationships][retainers] with some users that greatly informs me on what is needed. And reported issues also provide a way to see what is wrong. But a survey provides a new lens for identifying what is needed.

Sure, surveys have their own biases to deal with, such as self-selection. And voting in a survey costs very little, so each individual answer has less weight. But it helps quantify things we supposedly know from just a few users. Does this problem only show up in weird circumstances, or do most people run into it? Are there things that many people want, but generally don't get a chance to tell me?

## The results

We had a total of 206 unique respondents. I've broken up the results by theme and question below.

### Current usage

Many of the questions were about current usage patterns. To get more context on answers, to see what works, and also to see what people _don't_ mention using.

**Versions**: Nearly every single respondent is using hyper v1.x, but about 13% also continue to use the end-of-life v0.14.x.

**Roles**: Over 80% said they use hyper in a server, and the same amount in a client. 30% said they use both modes together as a proxy.

**Runtimes**: Unsurprisingly, 99% use hyper with the Tokio runtime. But 10% said they _also_ at times use a different runtime. Most commonly that's with `smol`, but io-uring runtimes made up a couple percent points.[^byor]

**TLS**: hyper provides the HTTP, and asks users to bring the S. 93% said they use rustls. 30% make use of native-tls, 15% openssl specifically, and it trails off from there.

**On top of hyper**: regarding the rest of the local ecosystem, 86% use reqwest, and 79% use Axum. About half selected Tower. For future surveys, it'd be very helpful to know if people use Tower without Axum or reqwest, or perhaps mentioned it because they just know its a dependency of Axum. A few wrote-in Tonic, which in hindsight, duh of course.

**Number of engineers**: half of respondents said they (and possibly a colleague) are the only ones at their company using hyper. Another 22% said 4-10, 12% said 11-50, and 17% said 50 or more. It's likely some places focus on higher levels, but many other companies have a lot of engineers working with hyper!

**Years of usage**: Nearly half of respondents have been using hyper for 1-3 years. 12% for less than a year, 30% for 4-6 years. And 11% said they've been using hyper for forever. Me too, me too...

**Industries**: 111 responses, which I grouped and sorted from most common to least. Software, cloud infrastructure, security. Finance, healthcare. Robotics, automotive, space, EV charging. Embedded. Media, streaming, events. Databases. Education. Government. Telecom, radio, chat. Geospatial. AI. Browsers. Games. Rust compiler.

It's just so humbling to think this code is helping in so many different places.

### Future work

Another set of questions were related to work we _could_ do in the future. This helps inform prioritization, and things we may not have thought of.

**Feature requests**: One of the questions came preloaded with common feature requests, and asked the respondents to rank them. Here's the accumulated ranking:

1. Metrics/Tracing/Events API
2. HTTP/2 Performance
3. Better Middleware
4. Add HTTP/3 Support
5. Documentation and guides
6. io-uring support

**Write-in features**: the follow-up question was a blank text box to write in any feature requests not in the above list. I grouped the common answers and sorted: legacy-client successor, original header casing/ordering, buffer control, improved errors.

One write-in suggestion wasn't really about a new feature, but keeps popping back up in my mind: how to get releases that are more stable.[^stable-releases]

**The hard parts**: We also asked what was hard about hyper. Grouped and sorted answers: integrating TLS, upgrading to v1, middleware complexity, advanced body streaming, combining all pieces for production.

**Topic ideas**: we asked for suggestions for topics for future blog posts and talks. This certainly doesn't need to be only from me, so if you see something interesting, fill the need! But after grouping the topics, they are: hyper internals, best practices, testimonials and success stories, security, tower usage, retries.

### Contributing

We also asked about some general contribution questions, to look at the contributor health of the project.

The standout here was: **what would help you to contribute more?**

The top response, a little over half of all respondents, said an improved contributing guide. Just a few percent under that want an updated roadmap. 20% said more responsive triage and reviews. And 15% said they could use increased mentoring.

## The insights

Looking it all over, this was awesome. I definitely to take some specific things with me. Well, sure, there's goodies all throughout the above results. But, at a high level, here's some thoughts, especially after combining it with other conversations I've had.[^not-everthing]

Of the technical things, I suspect I'll be spelunking in `h2` first, eking out performance refactors. It can use some love. Metrics is requested all the time, and it looks like I can collaborate on a design and review, but I believe there's a couple other contributors interested in the implementation. I do keep thinking about ways to configure a buffer pool or something. I think the extended testing idea has some potential.[^stable-releases] And I'd _love_ to see HTTP/3 finally land in hyper proper this year, at least as an unstable feature. I want the other things too, but... time, y'know? Though, as mentioned in the next paragraph, it'd be a win to onboard _others_ to work on and _own_ those things.

Improving contribution health, sustainable personnel of hyper, is high on my list this year. I've started working on a collaborator guide. I'm considering some new collaborators that have already been doing good work. I want to try GitHub's rotating auto-assign reviewers, to improve review times and spread review load. And I welcome help linking up our existing [ROADMAP][] into some more concrete and mentored issues.

## Outro

There it is. The hyper user survey 2025 results. Thank you to everyone who responded!

Does any of this interest you? Want to run with one of the ideas? [Join us][contribute]!

[^burnout]: This took a bit longer to write up than I meant. I have to admit, I felt some strong burnout at the beginning of the year. Sometimes being a maintainer is lonely. New year contract negotiations don't help. But I'm feeling better now.
[^byor]: Did you know you can [bring your own runtime](https://hyper.rs/guides/1/init/runtime/)? Also, this is one reason I don't believe the FUD about a single runtime. Also also, `hyper::rt` works fine, I don't see any need for the types to be in libstd. That wouldn't unblock anything.
[^stable-releases]: I can write this up more later, but initially, I've learned the slow way about feature windows. On top of those, I'd love to talk to customers who would be interested in having their test suites try hyper `master` before a release. [Talk to me][retainers].
[^not-everything]: I want to be clear: this isn't everything I plan to this year. This isn't a roadmap update. I think at a high level, the roadmap is still accurate. There's also plenty of security work to do. This is just meant to analyze the results and where they intersect with other priorities.

[survey-2025]: https://seanmonstar.com/blog/hyper-user-survey-2025/
[hyper]: https://hyper.rs
[retainers]: https://seanmonstar.com/retainer
[ROADMAP]: https://hyper.rs/contrib/roadmap/
[contribute]: https://hyper.rs/contrib/contributing/
