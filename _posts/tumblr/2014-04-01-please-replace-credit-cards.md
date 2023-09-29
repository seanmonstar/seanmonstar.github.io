---
layout: post
title: Please Replace Credit Cards
date: '2014-04-01T13:00:00-04:00'
tags:
- planet
- credit cards
- security
- passwords
- persona
- jwt
- bestof
tumblr_url: https://seanmonstar.com/post/81400378235/please-replace-credit-cards
---
Technology has greatly improved things this past decade. It’s peculiar that messaging nonsense has seen so much work, but something that quite literally costs people money continues to be so flawed. I’m talking about credit cards. It’s worth pointing out that I’m not a security researcher, just a concerned citizen.<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>

The flaw is a fundamental part of the design: for every charge, we must give the entire card number to the seller. That card number is **everything**. It gives all the information and power to charge as much money as the recipient wants. Holy carps. The sellers don’t charge as much as they want, because that would be illegal, and they’d lose their merchant account. Still, employees could keep the number and sell it. Or, more likely, as sellers keep a record of the number, hackers can steal them, and get all the moneys. Until you notice, report fraud, and the banks just swallow it.

One supposed fix is the [Swipe-and-PIN enhancement](http://www.theverge.com/2014/2/12/5405024/chip-and-pin-vs-chip-and-signature). This helps prevent copying of a card at a Point-of-Sale terminal. However, advances in magic (the Internet) have greatly _increased_ the amount of online shopping. Want a new spiked collar for Fluffles? Just open your browser, and **type in your credit card number**. Nothing to worry about, I’m sure they won’t record it. Oh what’s that? An email from Exotic Collars that their database was hacked, and they actually did have your credit card. Time to call the bank, and fix up all your auto-billing subscriptions.

### _You_ get a token, and _you_ get a token…

We can try to create rules around how to store credit cards, but just like passwords, it’s really hard to do correctly. Also, [just as with passwords](http://seanmonstar.com/blog/your-password-is-insecure/), merchants should never receive such powerful information in the first place. A solution could be providing the merchant a one time use token authorized for a specific amount for a specific merchant. The merchant charges the token, and it then becomes useless. It’s impossible to charge for more than agreed upon. Stealing the token is useless, because it only works for that merchant, it expires, and a credit account will only accept any token once.

If the source card or key which is used to generate tokens is compromised, a user can contact their credit provider, and generate a new private key. No merchants are affected, since they never had your key to begin with.

Recurring charges would be trickier. It’d require more cooperation among banks, standardizing some sort of unique account ID. Tokens could include the account ID, and merchants could safely hold onto that. When they need to make a new charge, they could request a new token for a certain amount. The user can approve the charge which sends a new token, or perhaps mark that a certain amount from a specific merchant every so often is auto-approved. I’m sure plenty of things could be done here to make the user experience easy. And there’s incentives for as easy as possible: Easier means users will spend more.

Credit accounts could provide apps to their users to make sending and approving tokens easy from our phones. Additionally, the app could also optionally prompt for approval when a merchant charges a token, to ensure there was no mistake or the token wasn’t somehow hijacked<sup id="fnref:2"><a href="#fn:2" class="footnote-ref" role="doc-noteref">2</a></sup>.

### Stand back, I don’t know crypto

I’m certain smarter people than myself could make a really secure design, but I’ll entertain you by stumbling around with mine. The implementation could look something like [Persona](https://developer.mozilla.org/en-US/Persona/Crypto). The tokens passed around could be JWTs. It could follow something like these steps:

1. A merchant could ask a user account for a token, including details like items purchased, total amount, and a merchant ID, with the blob signed by their private key.
2. The user sees the charge request, sees the details match the signed blob, and approves it.
3. The user’s account includes the original request, the user’s ID, and a signed blob using the user’s private key. This JWT would be sent to the merchant account.
4. The merchant account would then submit the token to credit company.
5. The credit company would verify the user’s blob against their current public key, and verify the merchant’s blob against their current public key.
6. Optionally request final approval from user.
7. Transfer specified amount of money from user’s account to merchant’s account.

It would take a lot of work to move the world over to this system, but the end result should be much more secure. It should mean much less fraud, and much fewer stories like [what recently happened with Target](http://pressroom.target.com/news/target-confirms-unauthorized-access-to-payment-card-data-in-u-s-stores). Can we please do this?

* * *

1. 

Or, I have no idea what I’m talking about.&nbsp;[↩︎](#fnref:1)

2. 

The design reduces the risk of a stolen token, since it’s generated for a specific merchant. However, it could be that a hacker gets control of a merchant account, or their private key, and can claim to be the merchant.&nbsp;[↩︎](#fnref:2)

