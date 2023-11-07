---
layout: post
title: Was async fn a mistake?
date: '2023-09-28T09:58-07:00'
tags:
- rust
- opinion
- async
tumblr_url: https://seanmonstar.com/post/66832922686/was-async-fn-a-mistake
---
This [stabilization PR for `async fn` in traits][stabilization] made me think: was `async fn` in Rust a mistake?

I mean, [I dunno][twit]. [Maybe it wasn't][masto]. But play along for a moment.

By the way, I don't mean that `async`/`await` in Rust itself is a mistake. That's a Big Deal. It allows companies to deploy some serious stuff to production. And `async` and `await` syntax is a huge save. I don't want to lose that. Writing manual futures and poll functions is megasad.

I'm specifically talking about the `async fn` sugar. What if we didn't have it, and instead just returned `impl Future`s, and used `async` blocks inside the functions?[^reading]

The current `async fn` is really nice, if you fit the expected usage. If none of the differences with `impl Future` ever cause you problems, then great! But I do run into them. Other people seem to also.

### What's so bad?

Some of these differences cause problems that don't have decent solutions. ([Do you know the differences?][play])[^me] If you have to deal with one of them, suddenly you need to use different syntax.

And now, **people need to understand both**. And keep the subtle differences in their head when they read. Does that make things better? Or worse?

It's the only place that has **a magic return type**. It makes lifetimes weird. With [suggestions to reign them in][lifetimes]. It leads to all sort of proposals about how to customize the return type. `#[require_send]`, `async(Send)`, `Service::call(): Send`, and I'm sure there's others.[^rtn] I also am thinking about generators and streams, since they could also end up with magic return values.

So was it mistake? I think it may have been. Don't worry, I don't want to take it away from you, if you disagree![^for]

### What if the alternative was nicer?

But I did wonder about this. What if we had the following features ready:

- [Repurpose bare trait syntax][bare] to mean `impl Trait`. It's been enough editions, right?
- Ability to forgo naming an associated type name.
- Stealing the feature from Scala where functions can _equal_ a single expression.

Then asynchronous functions could look like this:

```rust
fn call(&self, req: Request) -> Future<Response> = async {
    // ...
}
```

That'd be a nice improvement.

[^reading]: Yea, I know, it's a little more writing. But I am in the _optimize-for-reading_ camp. We read much more than we write. So if I have to write a few more characters at a function definition, but it makes the reading experience more understandable, that's a massive win.

[^me]: I've been involved in async Rust since the beginning. I know how it used to be, I was part of the group making it better, and I pay close attention to all the new proposals. I still mean what I said: _none_ of the solutions look nice.

[^rtn]: Return Type Notation (RTN) syntax is probably the least gross. But it raises a bunch of  questions. Does it work for all functions? If not, why not? If so, do I check `I::Iter` or `I::into_iter()`. And also to consider: Rust's [strangeness budget][]!

[^for]: I could see an argument that it's sort of like `for`, `while`, and `loop`. A more convenient syntax when it works, and you can use the others when you need more control. That argument breaks down when `async fn` is part of a trait definition. But anyways, I really just want the less-sugared way to be little nicer.

[stabilization]: https://github.com/rust-lang/rust/pull/115822
[twit]: https://twitter.com/seanmonstar/status/1702423803698286951
[masto]: https://masto.ai/@seanmonstar/111065463310520732
[play]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=beb7c7d6f540ca55c7d19795a023cdbe
[lifetimes]: https://ibraheem.ca/posts/extending-the-async-fn-syntax/
[bare]: https://rust-lang.github.io/rfcs/2113-dyn-trait-syntax.html#rationale-and-alternatives
[strangeness budget]: https://steveklabnik.com/writing/the-language-strangeness-budget
