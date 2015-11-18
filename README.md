# prime-challenge

This is a program built in response to a programming challenge we selected at
work. The goal was to take in a number, 0 >= 10,000,000 and output 4 primes that
add up to that number, otherwise output "Impossible!". You are allowed to repeat
primes.

## Why Rust

One of my immediately desires with this project was to take it to logical
extreme of optimization. This is why I decided to use Rust for it. The language
choice was completely open and I knew I'd likely want parallel solution to get
the fastest time that I could.

In addition to making parallel solutions easier to write, Rust is one of my
favorite languages to use. Finally, it has the nice feature of trying to compete
with C++ so it is no slouch when it comes to speed.

## Library Choice

Many challenges, folks opt to not use any libraries or limit their usage to
things outside of the core problem. I didn't take that path myself as I'm still
learning Rust and part of learning a language is working with other people's
code and libraries.

I decided from the start that I didn't want to implement a
[sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).
I've implemented them in the past and while they were interesting the first few
times, I opted to use a library to cover this for me. A quick check of libraries
available for primes led me to a pretty immediate answer:
[Primal](https://crates.io/crates/primal). This gives me the two things I'd be
implementing myself, caching and optimization.

Late in the project I wanted to add parallelism to my solution. In the past I've
just use the built in
[`spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html) and just build up
any additional logic myself. Recently though,
[Jim Blandy](http://www.red-bean.com/~jimb/) gave a talk at
[PDX Rust](http://www.meetup.com/PDXRust/) about using Rust, from the basics to
multiple threaded solutions with different libraries used for the threadpool.
One that he mentioned looked pretty simple so I opted for it:
[scoped_threadpool](https://crates.io/crates/scoped_threadpool). It worked
wonderfully and only took me 25-30 minutes to turn my solution from a single
threaded solution to a multithreaded solution using a pool and chunking out the
work.

