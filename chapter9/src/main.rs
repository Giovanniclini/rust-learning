use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // Error handling
    // Rust distinguishes between two types of errors:
    // 1. Recoverable errors: These are errors that you can handle and recover from, such as file not found or network issues.
    // 2. Unrecoverable errors: These are errors that you cannot recover from, such as accessing an out-of-bounds index in an array or dereferencing a null pointer.
    // In Rust, recoverable errors are handled using the `Result` type, which is an enum that can be either `Ok` or `Err`.
    // Unrecoverable errors are handled using the `panic!` macro, which will terminate the program and print an error message.

    // Unrecoverable errors
    // You can cause a panic in Rust in two ways: by directly calling the panic! macro 
    // or by taking an action that causes our code to panic.
    // By default, panics will print a failure message, unwind (Rust walks back up the stack and cleans up the data from each function it encounters), clean up the stack, and quit.
    // You can also immediatly abort, which ends the program without cleaning.
    // For example if you want to change abort on panic in release mode, add this in Cargo.toml:
    // [profile.release]
    // panic = "abort"
    // This will help making the binary as small as possible, because it will not include the unwinding code.

    // Example of panic
    //panic!("crash and burn");
    // This line generates:
    //thread 'main' panicked at src\main.rs:20:5: // Where it panicked
    //crash and burn // Panick message
    //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // $env:RUST_BACKTRACE = "1" in Windows Powershell to set the variable.

    // Recoverable errors
    // Result enum is defined as follows
    //enum Result<T, E> {
    //    Ok(T),
    //    Err(E),
    //}
    // T and E are generic types, T is the type of the value returned in case of success, and E is the type of the error returned in case of failure.

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // The type of the value that File::open returns inside the Err variant is io::Error, 
        // which is a struct provided by the standard library. This struct has a method kind that we can 
        // call to get an io::ErrorKind value. The enum io::ErrorKind is provided by the standard library 
        // and has variants representing the different kinds of errors that might result from an io operation.
        // We want to create a file if the file do not exist and 
        // panic if the error is not a NotFound error.

        // Closures can help us reduce the number of matches. (See chapter 13)
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // Unwrap or Expect
    // Using match can be a bit verbose and do not comunicate the intent very well
    // unwrap will return the value inside the Ok variant or panic if the Result is an Err variant.
    let greeting_file = File::open("hello.txt").unwrap();
    // Similarly, expect will return the value inside the Ok variant or panic if the Result is an Err variant, but it will also print a custom error message.
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
