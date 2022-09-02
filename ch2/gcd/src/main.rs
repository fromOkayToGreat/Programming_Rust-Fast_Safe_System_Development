use std::env; // this module provides several useful functions and types for interration with the execution environment of the program. The args function returns an iterator over the command line arguments that were passed to the program.
use std::str::FromStr; // this module provides the fromStr trait, any types that implement it, can be created from a string by calling the fromStr function.

fn main() {
    // main is the entry point of every Rust program.  The main function is special, it is the only function that is allowed to not have a return type specified.  In this program we ommit the `->` and return type entirely.
    let mut numbers = Vec::new(); // `mut` makes the variable mutable.  `Vec::new()` creates a new, empty vector.  The type of the vector is inferred from the type of the elements it contains.  In this case, the vector contains numbers, so the type of the vector is `Vec<i32>`.

    for arg in env::args().skip(1) {
        // here we are using a for loop to iterate over the command line arguments, setting the variable `arg` to each argument in turn and evaluating the loop body.
        numbers.push(u64::from_str(&arg).expect("error parsing argument")); // here we are using the `push` method to add a new element to the end of the vector.  The `push` method takes a single argument, the value to be added to the vector.  The `fromStr` function is called on the argument, and the result is added to the vector.  If the argument cannot be parsed as a number, the program will panic and display the message "error parsing argument".
    }

    if numbers.len() == 0 { // here we are using the `len` method to get the number of elements in the vector.  If the vector is empty, we print a usage message and return from main.
        eprintln!("usage: gcd NUMBER..."); // here we are using the `eprintln` macro to print a message to the standard error stream.  The `eprintln` macro is similar to the `println` macro, except that it prints to the standard error stream instead of the standard output stream.
        std::process::exit(1); // here we are using the `exit` function in the `process` module to terminate the program immediately.  The exit code passed to `exit` will be returned to the operating system as the program's exit code.
    }

    let mut d = numbers[0]; // here we are using subscript notation to get the first element of the vector.  The first element is assigned to the variable `d`.  If the vector is empty, this line will cause the program to panic.
    for m in &numbers[1..] { // here we are using a for loop to iterate over the elements of the vector, setting the variable `m` to each element in turn and evaluating the loop body.  The `&` operator is used to create a reference to the element.  The `..` operator is used to create a range that starts at the second element and continues to the last element.
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d); // here we are using the `println` macro to print a message to the standard output stream.  The `:?` placeholder is used to print the vector in debug format.  The `{}` placeholder is used to print the value of the variable `d`.
}

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

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
