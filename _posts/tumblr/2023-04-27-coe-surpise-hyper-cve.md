---
layout: post
title: Report on Surprise hyper CVE from 2023-04-11
date: '2023-04-27T15:49:36-04:00'
tags:
- hyper
tumblr_url: https://seanmonstar.com/post/715784167270596608/coe-surpise-hyper-cve
---
### Meta

This document is meant to help publicize the learnings from a recent emergency in hyper. Documents like these are common within various organizations. Some call them “postmortems”, others say “incident reports”. I quite like what Amazon calls them, since it aptly describes the purpose: **Correction of Error**. There was an error that caused an emergency, and we want to correct that error.

### Summary

A surprise [CVE](https://en.wikipedia.org/wiki/Common_Vulnerabilities_and_Exposures) publicly filed for [hyper](https://hyper.rs) on April 11, 2023 caused an emergency situation for several collaborators, and sent out dependabot warnings with no actionable advice. By day’s end, we identified a best-guess at what the cause of the low-severity vulnerability was. By the next morning, a fix was available.

That the issue should have been a CVE is uncertain.

The bigger concern is the way the CVE was filed bypassing the existing security policy. That is similar to finding a lighter in a school, and pulling the fire alarm. This COE discusses both why it may have happened, and how we can try to reduce future occurrences.

### The impact

The RustSec[^1] advisory explains the issue this way:

> If an attacker is able to flood the network with pairs of `HEADERS`/`RST_STREAM` frames, such that the `h2` application is not able to accept them faster than the bytes are received, the pending accept queue can grow in memory usage. Being able to do this consistently can result in excessive memory use, and eventually trigger Out Of Memory.

In reality, being able to consistently accomplish those conditions would be very difficult for an attack, and so the likelihood of this affecting anyone is minimal. Certainly low severity.

But the bigger impact was not this particular issue, but rather that a CVE caused a sudden panic for the maintainers and for users as dependabot alerted people with nothing that they could do.

### The story

The original [issue](https://github.com/hyperium/hyper/issues/2877) was filed on May 27, 2022. Trying to better understand, I asked some poorly worded follow-up questions. Another contributor filed a pull request trying to fix the underlying issue. Several collaborators reviewed that PR, but didn’t fully grasp what it was trying to fix. It then fell into the void.

On April 11, 2023, someone decided to file a public CVE for the described issue, without following the security policy. I commented on the issue that while the motivation for doing so was likely good-intentioned, it was the wrong way to go about it. GitHub imported the report, which started triggering dependabot warnings.[^2] This surprised us, and at least four people dropped everything to handle the fire alarm.[^3]

The first step was trying to determine a reproducible example. We didn’t notice at the time it was filed, but the original issue did not include full reproducible instructions. We tried to create some unit tests to mimic the behavior described, but couldn’t trigger the issue.

Eventually, we noticed that a modified test that stopped “accepting” requests from the connection, but still polled it, would cause the accept queue to grow. But hyper makes sure to have a task that is always accepting requests, unless you specifically ask it to stop. Thus, the modified test seemed like user error, but it was a just guess.

It just seemed too convuluted. Then we arrived at a much better guess.

We finally found a way to grow the accept queue even when continuously accepting, by creating a test to blast thousands of requests in a loop. Since `h2`s test suite uses in-memory IO streams, we are able to fill the read buffer to near infinity. That’s when we settled on our best guess: if someone can fill the socket’s read buffer faster than the server can pop requests, then the accept queue could grow unbounded. While there is a setting to limit concurrent requests, because these are immediately reset, the limit would never be checked.

After 14 hours, we had a fix written and reviewed. We determined that the issue was low severity, as the likelihood of being able to consistently attack was extremely low. And since we were adding a new limit, there was a possiblity of causing a new bug. So, better to not push something right before going to sleep.

The following morning we published the fix, as `h2` v0.3.17. Surprising everyone who has rushed out new code, a new bug in it was indeed found. We then published v0.3.18.[^4]

### Five whys[^5]

- **Why did someone file a CVE suddenly?** We don’t know for sure, but we can guess.[^6] A related issue had been open for a year, not fixed, so perhaps the reporter thought this was the only way to move forward.
- **Why wasn’t the issue acted upon a year ago?** When it was initially opened, the maintainers didn’t fully understand what the problem was. Follow-up questions were asked, but even our questions weren’t that clear. Eventually, we forgot about it.
- **Why was it forgotten?** We didn’t have any recurring reason to check back and try to understand what the issue was. If it had been reported privately to the security address, it would have stayed high priority until it was solved or determined incorrect.
- **Why wasn’t the initial issue reported privately?** Perhaps the original reporter didn’t know about the policy.

### What we’re doing to prevent a next time

We can’t completely control someone randomly filing a new CVE and causing another fire drill. But there are other things we can improve at to _reduce_ the likelihood of one.

- **Schedule routine triage.** This could be a synchronous meeting, such as in a text channel, or an audio channel. Or maybe over Twitch. But it can also just be a thing that [triagers](https://hyper.rs/contrib/governance/#triager) agree to do asynchronously, with a brief routine report to make sure we actually do it.
  - ⚠️ **If you or your company uses hyper** , this would be an especially useful way to help with maintenance. Have an engineer or two dedicate a few hours each month helping us triage.
- **Setup a bug report checklist.** There is a [triage guide](https://hyper.rs/contrib/issues/#triaging) for bug reports, which is a good thing. But that doesn’t mean everyone (me included!) always remembers all the steps. Checklists are famous in aviation and medicine for their effectiveness in saving lives. They can also help us make sure all issues are treated properly.
- **Update the issue templates to use forms instead.** We do have an issue template in place, to try to get people to fill in more information initially. But it’s pretty easy to skip it. It’s possible using GitHub’s new forms instead of just a text template could guide people more often.



[^1]: RustSec and the CVE database are different. RustSec was much more helpful, coordinating with us by waiting until the emergency panic was over, and then discussing the best way to describe the advisory.

[^2]: I updated the advisory on GitHub’s end to only indicate `h2`, not `hyper`. I also indicated my disappointment in GitHub’s amplifying of the alarm and making the day much more stressful. Their reply: “We do that sometimes XD”. Cool.&nbsp;[↩︎](#fnref:2)

[^3]: Meanwhile, a reddit thread took off, watching the action, commenting, and mostly criticizing the actions of all involved. Thankfully, I didn’t read comments like “I don’t have any sympathy for the maintainers” until after the fix was completed.&nbsp;[↩︎](#fnref:3)

[^4]: “At least this made you fix it, right?” No. This attitude is toxic. Doing it this way burns out everyone around who could fix it. There is a reporting process for a reason. It helps the most amount of people. Please use it.&nbsp;[↩︎](#fnref:4)

[^5]: Not literally five questions, but an [exercise to try to find the root cause](https://en.wikipedia.org/wiki/Five_whys), and to note any extra things that could be fixed along the way.&nbsp;[↩︎](#fnref:5)

[^6]: Some people tried to infer bad motives, such as for clout or “another notch on a security researchers belt”. I see no reason to assume that with no evidence.&nbsp;[↩︎](#fnref:6)

