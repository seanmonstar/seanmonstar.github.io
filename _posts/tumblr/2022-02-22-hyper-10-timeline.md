---
layout: post
title: hyper 1.0 timeline
date: '2022-02-22T13:15:14-05:00'
tags:
- hyper
- rust
- rust-lang
- http
tumblr_url: https://seanmonstar.com/post/676912131372875776/hyper-10-timeline
---
I said [earlier](https://seanmonstar.com/blog/hyper-ish-2021-in-review/) that this year we will release [hyper](https://hyper.rs) v1.0. hyper is the safe and fast HTTP library written in Rust. These are the steps to get there.

- **Define the vision.** Before anything else, we need to define what hyper even is. This will be an abstract vision showing what the goal and purpose of hyper is, based on what our users need.
- **Outline the roadmap.** Shortly afterwards, we’ll outline the major changes that are needed for hyper to line up with what was defined in the Vision.
- **Plan the work.** We’ll break down the required work into issues, labels and milestones. Issues should be reasonably worded to allow newcomers to join in the fun. This step should be done by the end of March.
- **Do the development work.** This is the coding and reviewing work to get hyper 1.0 ready. Our original plan won’t survive contact with the enemy, but we’ll adapt.
- **Ship a release candidate.** Part of the purpose of 1.0 is the stability promise. But since we’re making some changes, it’s wise to have release candidates so that some user feedback can still be incorporated before making things final. We aim to have the first RC out in the fall.
- **Perfect the documentation.** hyper 1.0 aims to have world-class documentation. Both on individual types, methods, and modules, and also guides that help explain how to do common things.
- **Gather feedback.** We want to hear how the release candidate is working out. Everyone trying it is encouraged to open new issues, but we’ll also reach out to various users to make sure we get the feedback we need. This period will likely be about three months, so people can migrate their code, give feedback, and allow us enough time to act on that feedback.
- **Ship it.** The launch. 1.0. All the confetti and excitement. But we won’t be done, there’s always things to do! This should happen in the middle of Q4 of 2022.

### How can I help?

It’s an exciting time! There’s a lot to do, and many different skills will be needed. If you want to be part of the effort, there’s [several ways to help](https://github.com/hyperium/hyper/blob/master/CONTRIBUTING.md)!

- Pick an issue and write code.
- Help us make the documentation as awesome as possible.
- Review pull requests as they appear.
- Provide feedback at any stage. The vision and roadmap will be pull requests that you can comment on. There will be issues with detailed designs. The release candidates need to be tried out, and we need to know any problems you have before we go the big one-oh.
- Chat with us in [#hyper](https://discord.gg/kkwpueZ).
