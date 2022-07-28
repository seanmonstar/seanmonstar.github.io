---
layout: post
title: Style Guide for Python Code
date: '2010-09-24T16:55:00-07:00'
tags:
- python
- planet
- programming
tumblr_url: https://seanmonstar.com/post/1181440279/style-guide-for-python-code
---
[Style Guide for Python Code](http://www.python.org/dev/peps/pep-0008/)  

I love Python; the style guide is a great resource for writing consistent looking code. I particularly enjoyed these:

- Always use `self` for the first argument to instance methods.

- `_single_leading_underscore`: weak “internal use” indicator. E.g. `from M import *` does not import objects whose name starts with an underscore.

- `single_trailing_underscore_`: used by convention to avoid conflicts with Python keyword

- `__double_leading_underscore`: when naming a class attribute, invokes name mangling (inside class `FooBar`, `__boo` becomes `_FooBar__boo`).

- Comparisons to singletons like `None` should always be done with `is` or `is not`, never the equality operators.

