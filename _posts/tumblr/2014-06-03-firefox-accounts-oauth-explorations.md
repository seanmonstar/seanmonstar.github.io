---
layout: post
title: Firefox Accounts OAuth Explorations
date: '2014-06-03T13:30:00-04:00'
tags:
- mozilla
- planet
- firefox accounts
- identity
- oauth
- gryphon
- fxa
- bestof
tumblr_url: https://seanmonstar.com/post/87709828215/firefox-accounts-oauth-explorations
---
As we build [Firefox Accounts](https://wiki.mozilla.org/Identity/Firefox_Accounts), a key part to the whole experience is allowing a user to divvy out information to apps. We’ll be doing so with an OAuthish experience.

### OAuth2

The first obvious place to look was the [OAuth2 spec](http://tools.ietf.org/html/rfc6749). We’ve based most of our experience on this model. Using the spec, a flow for an imaginary website Cuddly Foxes would like this:

1. Cuddly Foxes would register with our oauth server, supplying a `redirect_uri`, and we’ll give you back a `client_id` and `client_secret`.
2. Cuddly Foxes will make their login button redirect the browser to our server, passing the `client_id`, a random `state`, and desired `scope`, such as profile:email.
3. Our server will show the user some information about who is asking and what info is being asked for, and ask them to confirm. They can uncheck any scopes they don’t want to give out.
4. The server will generate a code representing the current auth request, and redirect back to Cuddly Foxes’ `redirect_uri`, including the random `state` and a `code` parameter.
5. Cuddly Foxes first verifies that the returned `state` is one they sent, and then sends the `code` back to our server, along with the `client_secret` they received at registration.
6. Our server will verify the `client_secret` matches what is associated with the `code`, and then will send back a `token` and the scopes the token has been approved for.
7. Cuddly Foxes would then use that `token` whenever asking for the user’s email address, from our profile server, since that’s the scope that was asked for.

So far, standard OAuth2. The `client_secret`, `code`, and `token` are 32-byte random hex strings, and we store a hash of them in our database, reducing the damage done if it gets compromised. Now, let’s add in another service provider: FoxCoin, the newest hotness in privacy-respecting crypto-currency. Cuddly Foxes wants to set up a recurring subscription to send new Foxes every month to users.

That means that they ask our OAuth server for a token with scopes ‘profile:email’ and 'foxcoin’. With the token in hand, they ask the Profile server for the user’s email, providing said token as proof that they can receive profile information, and they receive it. **But!** The profile server just received a token that it can use to access the user’s FoxCoin information, acting as Cuddly Foxes. Yikes!

Of course, we can assume the Profile server wouldn’t do anything so nefarious, but having that power is still dangerous. And imagine as we add in more 3rd-party attached services, which inherently are less trustworthy. Additionally, with the [recent discovery in OpenSSL](http://heartbleed.com/), we don’t want to trust TLS alone to protect against sniffing the data as it passes. So, passing around a Bearer token in plain text is unacceptable.[^1]

### OAuth2 HMAC

The next step was to consider using a secret token to sign a request, so that the original token is never revealed. This has been excellently explored already by the [Hawk scheme](https://github.com/hueniverse/hawk). The short of it is that 2 parties who share a secret can sign the request with an [HMAC](http://en.wikipedia.org/wiki/Hmac), proving that the request and it’s payload came from one of them. The receiver just computes the same HMAC, and compares signatures. The original secret is never leaked to anyone. Many cookies were had by all.

Adapting that to our OAuth flow, we would return a random token like before, and Cuddly Foxes would use it to generate a Hawk authorization header, and send it to our Profile server. The Profile server, not knowing the secret token, would tediously need to send the various bits of the request making up the signature, plus the authorization header, to our OAuth server. The OAuth server would look up the secret token, compute the HMAC, and return whether it was valid.[^2]

This is an improvement, since the secret token is never visible on the wire, nor does the Profile server receive it. However, a downside is that for this to work, the OAuth server needs to keep the original secret token in plain text. Before, we were keeping a hashed copy of it, which meant that a snapshot of our database would not reveal everyone’s secret tokens. We didn’t like this disadvantage, and so continued to explore.

### OAuth with Public Key Signing

We wanted to keep the request signature, since that doesn’t leak the secret to anyone else, while not having to retain the original secret ourselves. It turns out, there is a technology that does exactly this: asymmetric public key cryptography. However, using RSA or DSA keys has its problems: signing and verifying is slow, generating new keys is slow, and sending public keys with each request is a lot of bytes. That’s when my colleague [Brian Warner](http://www.lothar.com/blog/) brought up the newest hotness: elliptic curve public keys. Particularly, [Ed25519](http://ed25519.cr.yp.to/). It’s super fast to create keys, signing and verifying are fast, and public keys are 32-byte strings. The secret keys are likewise 32 bytes, and completely random, so brute force guessing takes longer than any human could ever wait.

So what’s that look like for Firefox Accounts? The updated flow looks like this:

1. Stays the same.
2. Stays the same.
3. Stays the same.
4. Stays the same.
5. Cuddly Foxes first verifies that the returned state is one they sent. They generate a new ed25519 keypair for this user+scope, and then sends the `pubkey`, the `code`, and the `client_secret` they received at registration to the server. This registers that public key with our OAuth service.
6. Our server will verify the `client_secret` matches what is associated with the `code`, save the public key, and return the scopes that have been approved.
7. Cuddly Foxes would then use that private key to sign a request asking for the user’s email address, from our Profile server, since that’s the scope that was asked for.

Afterwards, the Profile server can verify the signature by itself, since it contains the public key in it. This removes the need for each attached service to figure out what parts of a request to forward to the OAuth server. It also means that all service providers will handle their own hash computing, reducing strain on our OAuth server. Once a signature is verified, the Profile server can simply ask the OAuth server what scopes are provided for that public key, and then act accordingly.

Here’s an example request:

    HTTP/1.1
    GET /v1/email
    Host: profile.accounts.firefox.com
    Authorization: 'Gryphon pubkey="461d65b867d02ddf7f0d0bf3c2746c823605dec5e9f221ca7f451113fcddaf9f", ts="1400641081466", nonce="992022dd", sig="f1pIEz5y9sN6Bsc00iIy9YcEBFRLqCAtkTspvqQPb4FKUIMwrXxXiqBYXJbdAXc0FM1R6H9bdD+Pkx8klFUNCA=="'

The signature proves that the request originated from the owner of the pubkey, and the payload hasn’t been modified.

### There be Gryphons

The authorization scheme in the example above is “Gryphon”. It was partly influenced by Hawk, but felt like a more powerful version. Mozilla has a habit of naming projects after mythological creatures. Most importantly, [gryphons](http://en.wikipedia.org/wiki/Griffin) “are known for guarding treasures or priceless possessions.” Certainly, user data is a priceless possession.

[Gryphon isn’t complete](https://github.com/seanmonstar/gryphon). It’s currently in a proof-of-concept stage. There’s a [working branch of our oauth server](https://github.com/mozilla/fxa-oauth-server/pull/29) using it. However, we’d like to get more eyeballs on it before feeling confident about shipping. Are there missing pain points, or use cases not covered? Send me a [comment](mailto:comments@seanmonstar.com), or write up some analysis and [send me the link](https://twitter.com/seanmonstar), or come chat in [#fxa](https://wiki.mozilla.org/IRC), or anything, really.



[^1]: This issue doesn’t appear in all OAuth models. The issue comes from us having multiple mutually-distrusting services, being gated by our OAuth server. We plan to allow clients, such as your website, request data from a service provider, run by your digital neighbor, about a user.

In most cases, all the data comes from the same entity that runs the OAuth server, and so there’s no worry that it will mishandle the power it gives itself.

[^2]: A downside here is that this means the OAuth server is doing all hashing for all requests, which puts a requirement on our OAuth server using more resources.

