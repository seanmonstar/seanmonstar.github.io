---
layout: post
title: Make smaller structs
tags:
- rustlang
---
I've seen the hype about [view types](https://goals.rust-lang.org/2026/view-types-experiment.html) being added to Rust. I worry it exceeds the [strangeness budget](https://steveklabnik.com/writing/the-language-strangeness-budget/). And there's a solution already available. Make your structs smaller.

I've always insisted on _optimizing for reading_. And as many use LLMs make writing faster, there's even less reason to pick shorter but more complex.

Structs are free. They don't result in auto boxing. So, divide up your structs by how the fields relate to each other. Then it's much easier to borrow one field than to wish for some complex syntax to borrow many.

I have even created temporary projection structs to borrow disjoint fields together, though it's a pretty rare thing to need.

I wrote a [clippy lint](https://github.com/rust-lang/rust-clippy/pull/17129) to catch structs that are too big, but seems others think it's too strict. I still think it's a good idea.
