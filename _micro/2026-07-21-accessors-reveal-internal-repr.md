---
layout: post
title: Accessors can reveal internal representation
tags:
- rust
---
As a library maintainer, it's common to receive requests from users to add a "simple accessor". The data is _right there_. But that accessor can end up revealing internal representation details that you can no longer change.

This is one reason why it could be a better practice to return an `impl Iterator` instead of `&[T]`. The latter freezes you in place to always storing the data in consecutive memory. (Such as `String::as_bytes()`.)

View types might help too, like `Ref<'_>`. Returning a reference requires the data already exist in that shape in `self`. You can't change its shape in the accessor and return a reference to _that_.
