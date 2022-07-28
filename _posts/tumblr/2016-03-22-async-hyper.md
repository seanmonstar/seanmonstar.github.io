---
layout: post
title: async hyper
date: '2016-03-22T10:10:55-07:00'
tags:
- rust
- rust-lang
- mozilla
- planet
- hyper
- http
- programming
tumblr_url: https://seanmonstar.com/post/141495445652/async-hyper
---
It’s been a steady if somewhat slow march toward asynchronous IO in [hyper](http://hyper.rs). Over the past several months, I’ve tried out different strategies for implementing async IO. Eventually, after some useful conversations at last Mozilla All-Hands, I’ve settled on a performant approach using state machines. This reduces heap allocations, and eliminates all dynamic function calls inside hyper. This allows those who want the best possible performance to still be able to utilize hyper, while allowing various forms of asynchronous (and synchronous) programming styles to be built on top with minimal overhead.

**The good news: it’s nearly complete!** Of course, there’s bugs. And the API likely has rough edges. But hopefully framework developers can [start working with it now](https://github.com/hyperium/hyper/tree/mio), and help us have an excellent proper release real soon.

### State Machines

Here’s how it all works. A state machine type should be created, and should implement the `Handler` trait, altering its internal state as events from the socket occur, and responding to hyper with what it should do next.

    trait Handler {
        fn on_request(&mut self, req: Request) -> Next;
        fn on_request_readable(&mut self, decoder: &mut Decoder) -> Next;
        fn on_response(&mut self, res: &mut Response) -> Next;
        fn on_response_writable(&mut self, encoder: &mut Encoder) -> Next;
    }

There is a similar trait for the `Client`, but with the names and types adjusted.

The HTTP state is managed internally by hyper. The type that implements `Handler` should keep track of its business-related state, such as the request route, related headers, if data should be read, and when and if data should be written. The `Handler` conveys its desired intent by making use of the `Next` type that is returned after acting on events.

### Next

The `Next` type is used to declare what should be done with the current socket. The `Next` type has variants for `read()`, `write()`, `end()`, and less used forms.

#### Timeouts

The `Handler` can declare how long the server should wait for an event before triggering a timeout error. This would be asynchronously waiting, not actual blocking on IO.

Declaring a timeout is done with a method on the `Next` type:

    Next::read().timeout(Duration::from_secs(30))

The server would wait for a readable event on the socket for up to 30 seconds. If the event never occurs, then the `Handler` is notified with a `Error::Timeout`. Timeout errors can be handled in the `on_error` method of the `Handler`. The default implementation of this method is to abort the request.

    fn on_error(&mut self, err: Error) -> Next {
        match err {
            Error::Timeout => {
                // we could try to be good and repond with a 408
                self.code = hyper::StatusCode::TimedOut;
                Next::write()
            },
            _ => {
                // oh noes, just blow up
                Next::remove()
            }
        }
    }

#### Waiting

So far, the described API works well when the server can respond immediately to each event on a socket. But eventually a server starts adding other parts of the puzzle that aren’t available right away. There could be database connections, reading or writing to a file, proxying to another URL, or any other thing that would block the thread and put our event loop in stasis. In these cases, a `Handler` can receive events from the server, trigger other asynchronous actions, and notify the server when it is ready to finally do something. This is done using `Next::wait()` and the `Control` that is passed to every `Handler` upon construction.

    fn on_request(&mut self, req: Request) -> Next {
        match route(req.uri()) {
            Route::GetUser(id) => {
                let ctrl = self.ctrl.clone();
                User::find(id, move |user| {
                    // probably do something with `user`
                    ctrl.ready(Next::write());
                });
                Next::wait()
            }
        }
    }

In this example, after parsing a route such as `/user/33`, the `Handler` tells the server to wait (indefinitely) while we ask the database to look up the the user by ID. Once the database returns, the `Control` is alerted that the `Handler` is ready to write, and the server will continue processing the request.

### Breaking Everything

Due to the fundamental shift that exists when using non-blocking IO instead of blocking, there are several breaking changes. The most obvious is the change to the `Handler` trait, and having to deal with `io::ErrorKind::WouldBlock` errors when reading and writing.

Besides those biggies, several other parts of the API have been cleaned up for the better.

- `*res.status_mut() = NotFound` has become `res.set_status(NotFound)`
- The `Request` now has private fields, with only immutable accessors. This prevents mistakes since mutating the `Request` head would otherwise have no effect.
- The `HeaderFormat` trait has been merged back into the `Header` trait, with the help of `where Self: Sized`.

### hypersync

Everyone hates breaking changes. As necessary as they are, they will still inhibit some people from upgrading. To reduce some of the pain, I’ve worked on a “synchronous API” that is built on top of the new async hyper.

It mimics blocking IO on `read` and `write`, and allows the original `Handler` trait with the method `handle(&self, req: Request, res: Response)`.

A `hypersync` Server should be better protected from slowloris and DOS attacks, but it’s not going to be as performant as `hyper` itself, since it uses threads to provide the blocking IO feel.

Depending on the use case, many Clients don’t gain much benenfit from async IO, and may wish to use the synchronous programming style provided by `hypersync`.

It currently exists just as an [example in the hyper repo](https://github.com/hyperium/hyper/blob/mio/examples/sync.rs), but I hope to break it out into a separate crate that mimics most of the API of pre-async hyper.

