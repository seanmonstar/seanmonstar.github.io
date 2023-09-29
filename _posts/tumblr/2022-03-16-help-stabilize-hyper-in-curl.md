---
layout: post
title: Help stabilize hyper in curl
date: '2022-03-16T11:44:50-04:00'
tags:
- rust
- rust-lang
- hyper
- http
- curl
tumblr_url: https://seanmonstar.com/post/678895803144830976/help-stabilize-hyper-in-curl
---
We’ve been working for that past year and change to allow [hyper to be an HTTP backend for curl](https://seanmonstar.com/blog/how-using-hyper-in-curl-can-help-make-the-internet/). **We’re so close to having it work!** With hundreds of tests working, there’s only a dozen or so tests left to fix. I’ve created a [**dashboard**](https://github.com/orgs/hyperium/projects/2/views/1) based on the [actual file](https://github.com/curl/curl/blob/master/tests/data/DISABLED).

**Want to help us get it over the finish line?** I’ve written up a guide of how to help debug these tests, step-by-step<sup id="fnref:1"><a href="#fn:1" class="footnote-ref" role="doc-noteref">1</a></sup>, and then included an example of a test I debugged.

### Build hyper’s C API

First, we’ll build `libhyper`. Start by checking out hyper with git:

    $ git clone https://github.com/hyperium/hyper
    $ cd hyper

Next, we build the C library specifically, following the [instructions](https://github.com/hyperium/hyper/blob/master/capi/README.md):

    $ RUSTFLAGS="--cfg hyper_unstable_ffi" cargo +nightly rustc --features client,http1,http2,ffi -Z unstable-options --crate-type cdylib

### Build curl with hyper

Now to use that to build curl. Get back to your normal code directory, and checkout curl:

    $ git clone https://github.com/curl/curl
    $ cd curl

Make sure you have the [required software already installed](https://github.com/curl/curl/blob/master/GIT-INFO). In my case, I needed to install autoconf and libtool.

Then, follow the [in-repo instructions](https://github.com/curl/curl/blob/master/docs/HYPER.md):

    $ ./buildconf
    $ ./configure --with-hyper=<full path to hyper dir>` --disable-shared --enable-debug 
    $ make
    $ LD_LIBRARY_PATH="<hyper dir>/target/debug:/usr/local/lib" make test

When running `./configure`, you’ll also need to select a TLS option. Since I was on Linux, I added on `--with-openssl`. For ease, you can pick `--with-secure-transport` on macOS, `--with-schannel` on Windows, or pick any of the other options if you know better. I wanted to be able to try the built curl on the regular web, but if you don’t care about that, you can also just use `--without-ssl` to skip the TLS decision entirely.

### Pick a test from the list.

Look at the[dashboard of remaining tests](https://github.com/orgs/hyperium/projects/2/views/1).

### Debug it!

Once you have the test number picked, you can run it specifically:

    $ cd tests
    $ LD_LIBRARY_PATH="<hyper dir>/target/debug:/usr/local/lib" ./runtests.pl <test number> -f

And… it will have failed. But we expected that! Now for debugging. Run it again with the debugger flag `-g`, which will use `gdb` to help us debug:

    $ LD_LIBRARY_PATH="<hyper dir>/target/debug:/usr/local/lib" ./runtests.pl <test number> -f -g

When prompted, enter `run` (or just `r`). Pay attention to output to give hints about where look in the code. Some useful gdb tips:

- `break c-hyper.c:<line-no>` to set a breakpoint for a specific line in the [`c-hyper.c`](https://github.com/curl/curl/blob/master/lib/c-hyper.c) file.
- `n` for the next line after breaking.
- `p <expression>` to print the value of an expression inside the function.
- `c` to continue (unbreak).

It’s a cycle of paying attention to the output, looking in the source code, checking the documentation, setting or adjusting breakpoints, and running again. Eventually, you can figure out what went wrong.

### An Example

I randomly picked unit test 670 and ran it in the debugger. I saw this interesting line of output:

    * Hyper: [1] error from user's HttpBody stream: body write aborted

I realized that error message was a bit vague, so I updated it in hyper, and a re-run now shows this more useful message:

    * Hyper: [1] user body write aborted: early end, expected 49 more bytes

The request body is emitting some bytes, and then returning that the body is ended, while hyper thinks that 49 more bytes are needed to meet the advertized `content-length` header.

Since this unit test is about pausing the request body writing, it seems that instead of pausing, curl is signalling an end of the body. Just to double check, I found the function in `c-hyper.c` that returns `HYPER_POLL_READY` and body chunks, set a breakpoint, and observed it returning done because of a user call to pause.

So, the action items from debugging this are:

- [File an issue with hyper](https://github.com/hyperium/hyper/issues) to expand the [docs](https://docs.rs/hyper/latest/hyper/ffi/fn.hyper_body_set_data_func.html) to say more about pausing. They do say some, but it’s easy to miss.
- [File an issue with curl](https://github.com/curl/curl/issues) about the need to adjust how bodies are paused.
- **Bonus** : fix the code and send a pull request. But if you’re not familiar with C, even just the issues identifying the underlying problem help immensely!

* * *

1. 

The exact steps could change in the future! &nbsp;[↩︎](#fnref:1)

