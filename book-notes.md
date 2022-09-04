# Chapter 1: Systems Programmers Can Have Nice Things

Assuming that C and C++ programmers can avoid undefined behavior in their code, is like assuming a game of chess can be won by simply knowing the rules. They are put in an awkward position as these are the industry standard languages for systems programming.

## Rust Shoulders the Load for you

The Rust language make you a simple promise: If your program passes the compiler’s checks, it is free of undefined behavior. Dangling pointers, double-frees, and null pointer dereferences are all caught at compile time. Rust imposes more restrictions on your code, and these take practice and experience to get used to. Rust aims to be both safe and pleasant to use. Trusting the language to catch more mistakes encourages us to try more ambitious projects.

## Parallel Programming Is Tamed

Some of the restrictions that ensure memory safety also ensure that Rust programs are free of data races. Unchanging data can be freely shared between threads. Data that does change can only be accessed using synchronization primitives. Many traditional concurrency tools are available: mutexes, condition variables, channels, atomics, and so on. Rust just wants to make sure you are using them properly.

## And Yet Rust Is Still Fast

Rust is designed with efficient defaults and gives you the ability to control how memory gets used and how the processor’s attention is spent.

## Rust Makes Collaboration Easier

Rust’s package manager and build tool, Cargo, makes it easy to use libraries published by others on the Rust’s public package repository, the [crates.io](http://crates.io) website. It’s sound on version management and reproducible builds.

The language itself is designed to support collaboration: Rust’s traits and generics let you create libraries with flexible interfaces that can endure many different contexts.

# Chapter 2: A Tour of Rust

## `rustup` and `cargo`

### `rustup update`

Rust’s package manager, `rustup`, is used to install and update Rust. It also manages the toolchain, which includes the compiler, `rustc`, and the package manager, `cargo`.

### `cargo`

The `cargo` command is used to build and manage Rust projects. It is the primary tool you will use when writing Rust programs. You can use Cargo to start a new project, build your project, and manage any external libraries your project depends on.

### `rustc`

The `rustc` command is used to compile Rust programs. Usually we let `cargo` handle this for us, but it is useful to know about `rustc` when we want to control how the compiler works.

### `rustdoc`

The `rustdoc` command is used to generate documentation for Rust projects. If you write documentation in comments of the appropiate form in your program's soure code, `rustdoc` can build nicely formatted HTML from it.

## Hello, World!

```
$ cargo new hello
     Created binary (application) `hello` package
```

> This command creates a new package diretorry called `hello` and generates a new project with a `Cargo.toml` file and a `src/main.rs` file.  If not inside a Git repository, `cargo` will also create a `.gitignore` file.


Cargo by default creates a binary project.  The `src/main.rs` file is the entry point of the program.

```fn main() {
    println!("Hello, world!");
}
```
> The `main` function is the entry point of the executable.  The `println!` macro is used to print text to the screen.

The file **Cargo.toml** is the manifest file for the project.  It contains information about the project, such as the name, version, authors, and license.  It also contains a list of dependencies for the project. The `[dependencies]` section is where we list the dependencies for our project.

```
[package]
name = "hello"
version = "0.1.0"
authors = ["Your Name <
>"]
edition = "2018"

[dependencies]
```

### `cargo run`

The `cargo run` command builds your project and then runs the resulting executable.  Cargo places the executable in the `target/debug` directory.  The `target` directory is where Cargo stores all of the intermediate artifacts and final build products.

```
$ cargo run
   Compiling hello v0.1.0 (/home/you/projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/hello`
Hello, world!
```
### `cargo clean`

The `cargo clean` command removes the `target` directory and the `Cargo.lock` file.  The `Cargo.lock` file stores information about the exact versions of dependencies that were used in the last build.  This file is used to ensure that the same versions of dependencies are used in every build.

## Rust Functions

Here is a function that computes the greatest commond divisor of two integers using Euclid's algorithm.

```
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
```
> The `gcd` function takes two unsigned 64-bit integers as parameters and returns an unsigned 64-bit integer as a result.  The `assert!` macro is used to check that the parameters are not zero.  The `while` loop is used to compute the greatest common divisor.  The `if` statement is used to ensure that `m` is always greater than `n`.  The `let` statement is used to create a new variable, `t`, that is used to swap the values of `m` and `n`.  The `m = m % n` statement is used to compute the remainder of `m` divided by `n`.  The `n` at the end of the function is used to return the result.

- In pratice, most variables don't get assigned to; the `mut` keyword allows the variable to be assigned to later.
- The ! character after the `assert!` macro indicates that it is a macro invocation rather than a function.
- Rust's `assert!` macro will termiante the program if the condition is false and print a message to the screen, this kind of termination is called a *panic*.
- There's also a `debug_assert!` macro, whose assertion are skipped when the program is compiled in release mode (for speed).
- Rust does not require parenthesis around the condition of an `if` statement, but it does require braces around the body of the `if` statement.
- The `let` statement is used to create a local variable, like `t` in the `gcd` function.  We don't need to write out the type of the variable, Rust can infer it from the value we assign to it.  **Rust only infers types within a function body.**
Rust has a `return` statement, but the last expression in a function is implicitly returned.  If a finctions body ends with a expression that is not followed by a semicolon, the value of that expression is returned.
- Any block sorrounded by curly braces is an expression, and the value of the block is the value of the last expression in the block.

For example, the following function returns the value of the last expression in the block.

```
fn foo() -> i32 {
    let x = 5;
    x
}
```

**It's typical in Rust ot use `snake_case` for function and variable names and `CamelCase` for types, traits, and modules.**

## Writing and Running Unit Tests

Unit tests are small and more focused tests that test one module in isolation at a time.  Unit tests are useful for testing private functions and for ensuring that public functions behave as expected.  We can have test functions scatettered throughout our code, but it's more common to put all of the test functions in a single module named `tests` in the same directory as the code they are testing.

```
#[test]
fn test_gcd() {
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}
```

> The `#[test]` attribute indicates that this is a unit test.  The `assert_eq!` macro is used to check that the result of the `gcd` function is equal to the expected result.

The #[test] marks the `test_gcd` as a test function, to ber skipped in normal compilation, but included and called automatically when we run `cargo test`.  It's an example of an attribute.  Attributes are a open-ended system for marking functions and other declarations with extra information.

## Handling Command-Line Arguments

The `std::env` module provides functions for working with the program's environment, including the command-line arguments.  The `args` function returns an iterator that produces a sequence of command-line arguments.  The first argument is the path of the program, and the remaining arguments are the arguments that were passed to the program.


Ex.1
```
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: hello NAME");
        std::process::exit(1);
    }
    println!("Hello, {}!", args[1]);
}
```
> The `args` function returns an iterator that produces a sequence of command-line arguments.  The `collect` method consumes the iterator and collects the arguments into a vector.  The `len` method is used to check that the program was called with exactly one argument.  The `eprintln!` macro is used to print an error message to the screen.  The `exit` function is used to terminate the program with an error code.  The `println!` macro is used to print the greeting to the screen.


## Serving Pages to the Web

A Rust package, wether a library or an exacutable, is called  a *crate*.  A crate is a collection of Rust source files.  The `Cargo.toml` file is used to configure a crate.  The `Cargo.lock` file stores information about the exact versions of dependencies that were used in the last build.  This file is used to ensure that the same versions of dependencies are used in every build.

**See example: `ch2/actix-gcd`**

## Concurrency

The rules tha ensure that Rust programs are free of memory erros also ensures that threads can share memory only in ways that avoid data races.

- Using a mutex to coordinate threads making change to a shared data structure, Rust ensures that you can't access the data except when you're holding the lock, and releae the lock automatically when you're done.
- Transfering data ownership from one thread to another, Rust ensures that all access to it has been relinquished before the data is used in the new thread.
- Sharing read-only data among several threads, Rust ensure that you cannot modify the data accidentally.

**See example: `ch2/mandelbrot`**

