---
layout: post
title: hyper 1.0 roadmap
date: '2022-04-06T09:45:33-07:00'
tags:
- hyper
- http
- rust
- rust-lang
tumblr_url: https://seanmonstar.com/post/680802159018803200/hyper-10-roadmap
---
> hyper is a protective and efficient HTTP library for all.

We have a [timeline](https://seanmonstar.com/2022/07/28/2022-02-22-hyper-10-timeline.html) for how to get to [hyper](https://hyper.rs) 1.0, and a [vision](https://github.com/hyperium/hyper/pull/2772) of what it looks like. Next up is a proposal for how to get there. That’s this new [roadmap](https://github.com/hyperium/hyper/pull/2806).

The purpose of the roadmap is to map out what hyper looks like now, and exactly how it needs to change to match up with the [VISION](https://github.com/hyperium/hyper/pull/2772).

The roadmap starts by describing current known issues with hyper v0.14, after having spoken to many users.

Then, after using the tenets defined in the vision to prioritize and make decisions, the proposed changes are listed. A central theme is making hyper stable, flexible, and ready for the future.

The roadmap includes a growable FAQ in the appendix. As the proposal receives feedback, the answers to that feedback should usually result in documenting it in that FAQ. There’s one question in particular that I want to inline here:

> ### Isn’t this making hyper harder?
> 
> We are making hyper more **flexible**. As noted in the [VISION](https://github.com/hyperium/hyper/pull/2772), most use cases of hyper require it to be flexible. That _can_ mean that the exposed API is lower level, and that it feels more complicated. It should still be **understandable**.
> 
> But the hyper 1.0 effort is more than just the single `hyper` crate. Many useful helpers will be migrated to a `hyper-util` crate, and likely improved in the process. The [timeline](https://seanmonstar.com/2022/07/28/2022-02-22-hyper-10-timeline.html) also points out that we will have a significant documentation push. While the flexible pieces will be in hyper to compose how they need, we will also write guides for the [hyper.rs](https://hyper.rs) showing people how to accomplish the most common tasks.

Please give us [feedback](https://github.com/hyperium/hyper/pull/2806). This is a joint effort. Only together can we make HTTP safer for all. After the roadmap is merged, we’ll file [issues](https://github.com/hyperium/hyper/issues) for all the pieces of work, to allow more fine-grained discussion, and to track the progress. Exciting times are ahead!

