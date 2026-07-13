---
layout: post
title: Rust Errors generally shouldn't be PartialEq
---
Errors might have internal details that don't make sense to compare. Like positional data, or a source chain. Even if you don't have those details yet, committing to a public API of `PartialEq` can restrict you from internal refactoring.

The common reason to want this is for testing purposes. People want to assert an error case matches what they expect. But that doesn't need equality. `matches!` , `kind()`, or `is_*()` patterns fit better.

A public, exhaustive, flat `enum` could choose differently. But those look more like a `kind()`. If you have private error details, I don't think they make sense for comparing.

Some interesting discussion a few years ago: https://www.reddit.com/r/rust/comments/n12uf9/is_there_a_good_reason_not_to_impl_eqpartialeq/
