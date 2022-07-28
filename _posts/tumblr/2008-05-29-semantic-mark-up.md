---
layout: post
title: Semantic Mark-up
date: '2008-05-29T10:23:00-07:00'
tags:
- html
- standards
tumblr_url: https://seanmonstar.com/post/706940006/semantic-mark-up
---
We see valid code popping up all over the place. And that’s great! Web standards should be followed by everyone. But, is it possible to make your code pass W3C’s validator without actually meeting the standards?

#### Valid vs Semantic

Standards were set for xHTML, and the validator can read through and make sure that every textarea tag has the properties for cols. But what if you use tags improperly? You could very easily make a heading, without using a h2 tag. Or you could italicize text, without using the em tag. What does that hurt?

You’ve lost the strength of web standards in relation to better SEO, because spiders collect data in headers as more important than in paragraphs, and you’ve lost all emphasis for accessibility. Reading software checks for headings, paragraphs, emphasis, strong, lists, and so forth. So the question is, how’s the validator supposed to know the difference?

It can’t!

#### Examples of Semantic Mark-up

Here’s some examples of non-semantic versus semantic mark-up, as well as some tips to doing so.

##### The Logo

You could make the logo appear by styling a div to contain the logo.

    \<div id="logo"\>\</div\>

But think about it in a semantic perspective. What does the logo mean? Is it the most important distinguisher on the page? Well then, give it an h1 tag! And on the web, wouldn’t the logo likely represent the whole web-site as a whole? Make sure it’s got an anchor to the index as well.

    \<h1 id="logo"\>\<a href="http://domain.com/" title="Domain's Home Page"\>Domain Dot Com, Inc\</a\>\</h1\>

Make sure to use a good title as well, and also spell out the company name inside the h1 tag. Since the readers and spiders couldn’t care less about your logo, you need to tell them what is important in text format.

##### Paragraphs

I’ve seen code like this before:

    \<div class="main-text"\>Welcome to McArthur GFX, the portfolio of Sean McArthur. He currently lives in Orange County, CA.He's a designer, developer, programmer, and CSS genious. He plays with Rubik's cubes but sucks at it.\</div\>

Obviously, this is standard text on a web-site, and should be inside a paragraph tag. You could simply change the div’s to p’s, and keep the same style. It will look exactly the same. So why do it?

Like I’ve said, outside the p tag, text readers aren’t going to know what proper emphasis to put on the text. Bad for SEO. Bad for accessibility.

##### Emphasis

Take the header quote of my web-site. It has text in there, possibly “I live a web-standard world.” And I’ve changed the color of web-standard to a reddish. Now here’s two ways you could do this, the second being the better way.

    \<h2\>I live in a \<span class="red"\>web-standard\</span\> world.\</h2\>

And the semantic way:

    \<h2\>I live in a \<em\>web-standard\</em\> world.\</h2\>

You can use ths span tag to differentiate text in the same in-line paragraph, but what is the purpose? Is it merely for design purposes? In my case, I wanted to emphasis the “web-standards” phrase. I mean, I’m really wanting it to have the same power as italics, but I want it red instead. So when read, it should still have the emphasis of the em tag.

##### Navigation

You could make your navigation simply a set of divs that each contain their own anchor and background image. And that would validate. But, you have to think about what the navigation is in terms of structure. Your site navigation is really a _list_ of links.

    \<ul id="nav"\> \<li\>\<a href="/"\>Home\</a\>\</li\> \<li\>\<a href="/contact/"\>Contact\</a\>\</li\>\</ul\>

#### Additional Tips

1. **Subscripts or fine print:** Use the \<small\> tag.
2. **Describing form elements:** Use that \<label\> tag. It let’s readers know it’s labeling a form element.
3. **Quoting elements:** There’s the \<blockquote\> to make something stick out in it’s own block, and there’s \<cite\> to quote something from somewhere else.

The key here, is to realize that in the xHTML, you’re creating the informational structure. The HTML is what crawlers, readers, and the like care about, and all these useful tags were created for certain data types. You can always change the way everything looks with CSS. You don’t like the way cite makes things italics? Then change it up. But keep it cite, because that’s its real data type.

