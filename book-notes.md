# Chapter 1: Systems Programmers Can Have Nice Things

Assuming that C and C++ programmers can avoid undefined behavior in their code, is like assuming a game of chess can be won by simply knowing the rules.  They are put in an awkward position as these are the industry standard languages for systems programming.

## Rust Shoulders the Load for you

The Rust language make you a simple promise: If your program passes the compiler’s checks, it is free of undefined behavior.  Dangling pointers, double-frees, and null pointer dereferences are all caught at compile time.  Rust imposes more restrictions on your code, and these take practice and experience to get used to. Rust aims to be both safe and pleasant to use.  Trusting the language to catch more mistakes encourages us to try more ambitious projects.

## Parallel Programming Is Tamed

Some of the restrictions that ensure memory safety also ensure that Rust programs are free of data races. Unchanging data can be freely shared between threads.  Data that does change can only be accessed using synchronization primitives.  Many traditional concurrency tools are available: mutexes, condition variables, channels, atomics, and so on.  Rust just wants to make sure you are using them properly.

## And Yet Rust Is Still Fast

Rust is designed with efficient defaults and gives you the ability to control how memory gets used and how the processor’s attention is spent.

## Rust Makes Collaboration Easier

Rust’s package manager and build tool, Cargo, makes it easy to use libraries published by others on the Rust’s public package repository, the [crates.io](http://crates.io) website.  It’s sound on version management and reproducible builds.

The language itself is designed to support collaboration: Rust’s traits and generics let you create libraries with flexible interfaces that can endure many different contexts.