---
layout: post
title: hyper Polish Period
date: '2022-11-15T12:43:21-05:00'
tags:
- rust
- rustlang
- hyper
- http
tumblr_url: https://seanmonstar.com/post/701008919383932928/hyper-polish-period
---
A couple weeks ago, we announced the [first release candidate](https://seanmonstar.com/blog/hyper-v100-rc1/) of hyper 1.0. [hyper](https://hyper.rs) is a protective and efficient HTTP library for all, written in the Rust programming language.

I’m calling the time period between the first release candidate and the final release the “hyper polish period”.

### Areas of Polish

The main thrust is to polish the edges, and make the final release of hyper 1.0 as smooth as possible for all. It’s still quite a lot to do, and several of the areas will actively benefit from a mixture of experience helping out. It seems there’s four main areas of work to do, so we need a few things from the community:

- Folks to volunteer to “lead” an area.<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>
- Anyone to pick a task from an area and work on it.
- Try to _use_ the release candidates, and give us feedback.

How about some more details about each area.

#### Area: Docs

hyper should have stellar documentation, all around. We should be an example for all Rust crates on how to have top-notch documentation. Caring about the documentation is caring about the developer experience. They are _our_ users. They should be delighted whenever trying to do something new with hyper. Certainly, there will always be ways to improve them. But that doesn’t mean we can’t try to make them awesome for the 1.0 launch.

The API documentation needs to better rounded out. Behavior should be explained here, a user shouldn’t _need_ to go read a guide to understand if a type has a certain behavior.

The website, [hyper.rs](https://hyper.rs), can be improved to better accomplish its various purposes. The examples from the repository could be rendered on the website. The `docs` folder containing internal documentation could be rendered as a “contrib” section. It could be easier to find the blog (and we could use that more).

While [guides](https://hyper.rs/guides) are technically part of the website, I view them to be sufficiently big enough to consider separately. We could use a more in-depth guide using more of hyper, perhaps slowly building on pieces from previous guides. WIth so many useful pieces now in `hyper-util`, guides should help show how to include and use them. We can also add a Tower section, showing the power of adding Tower middleware to your server of client.

#### Area: Utils

Many of the less stable, higher level parts of hyper 0.14.x have been removed, with the promise of most showing back up in [`hyper-util`](https://github.com/hyperium/hyper-util). Some of these are very simple ports. Others will be made more generic or configurable, encouraging users to plug and play on top of it all.

For example, consider the previous `hyper::Client` . It combined many concepts together for the convenience of users: a mechanism to establish new connections, a pool to store idle connections for a period of time, and a way for the connector to signal to the pool if HTTP/2 was negotiated via ALPN. You could only customize those pieces if specific options were part of the client `Builder`. As part of the move to `hyper-util`, the way those pieces plug together can be made public.<sup id="fnref:2"><a href="#fn:2" class="footnote-ref" role="doc-noteref">2</a></sup> That does likely mean more work on our part though.

#### Area: Upgrading

We want as smooth of an _upgrade_ as possible.

The smoothest would be if there were _only_ types or methods being removed. Then we could just add deprecations in 0.14.x, and the compiler would help people prepare early. But, when an upgrade involves _changes_ to methods, or behavior, it’s more complicated.<sup id="fnref:3"><a href="#fn:3" class="footnote-ref" role="doc-noteref">3</a></sup> Still, the more we can do to make things easier, the better!

As described in the [upgrade meta issue](https://github.com/hyperium/hyper/issues/3052), we can backport some of the new additions, and we can use `#[deprecated]` to guide people to start using better stuff earlier.

We can only backport _additions_, since that won’t break code. We can’t backport removals, or changes. In some ways, this work is on the easier side, since it’s not solving _new_ problems. It’s taking copying working code from the release candidate, and pasting it into the `0.14.x` branch. Likely some internal types or methods have changed names.

It does mean in some cases including significantly more code, so we’ll make the backports opt-in with a Cargo feature flag.

Once some of the new APIs are backported, we can add deprecation messages to the things they are meant to replacing. They can include a link back to a specific issue describing the motivation and anything else they need to consider about the change.

There are other parts of that are removed completely, without a replacement in core `hyper`. It seems like it would be useful to let people start preparing for their disappearance. Each deprecation warning can point to its own issue, explaining what the user could do instead, perhaps changing the pattern they use, or copying the code of what’s in `hyper-util`.

I don’t expect everyone to upgrade to 1.0 as soon as it comes out, so having the compiler help people to be more ready and make their eventual upgrade smoother has significant value. It helps earn the trust of our users, hopefully reducing upgrade fear for future releases.

#### Area: Integration Feedback

The release candidate period is principally to allow for gathering of feedback about any further breaking changes that should be considered before stabilizing.

Some users have _already_ tried out the release candidate, and filed issues of things we need to address. But we can’t simply assume that if no one complains, people like it. More often, if someone doesn’t like something, they just quietly use something else entirely.

We also can’t assume people will even try it out _on their own_ during this time period.

So, we’ll want to actively engage people with different [use cases](https://github.com/hyperium/hyper/blob/v1.0.0-rc.1/docs/VISION.md#use-cases), help them try to upgrade, and record their feedback ourselves. The more reports we can gather, the better. I also have a simple list of people at big companies, start-ups, and hobbyists that I’ll be sure to check in with.

If you’re able to help _give_ the feedback, we recognize the time and effort to do so as a valuable contribution towards the hyper project. The project and community are made better because of you, so thank you!

This may also reveal changes that would be good to make in `hyper-util`. But releases there are easy. The most urgent part of this area is to make sure we take care of all necessary breaking changes in `hyper` core, before calling it 1.0.

### Join us!

We expect this polish period to last a couple of months. There’s a lot to do, and a lot of places for people of all sorts to join in and make this release awesome.

Check out the boards, implement [something cool](https://github.com/orgs/hyperium/projects/1/views/7), write a [guide](https://github.com/orgs/hyperium/projects/1/views/8), give us [feedback](https://github.com/hyperium/hyper/issues), [backport something](https://github.com/orgs/hyperium/projects/1/views/9), volunteer to [own](https://discord.gg/kkwpueZ) (or co-own) an area. You can make a big difference for users of Rust’s HTTP libraries.

1.0, here we come!

* * *

1. 

Volunteering to lead an area doesn’t mean you’ll be the sole person responsible. Multiple can do so.&nbsp;[↩︎](#fnref:1)

2. 

Since the client pieces are not part of the stable `hyper` core, we don’t have to be quite as careful about not exposing some internals. It’s less of an issue to release breaking changes of utilities.&nbsp;[↩︎](#fnref:2)

3. 

I suppose it is doable by stretching those kinds of changes out over 2 major releases. You could add a stop-gap method and deprecation on the original the current version, then in the next make the breaking change and add a deprecation to the stop-gap method, and then remove it in the second release. hyper isn’t at that level of stability just yet, but something we could consider after 1.0.&nbsp;[↩︎](#fnref:3)

