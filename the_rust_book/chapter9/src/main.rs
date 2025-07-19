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


    // Propagating errors
    // When a function's implementation calls something that might fail,
    // instead of handling the error, it can return a the error to the caller.
    // See function read_username_from_file below.
}

// This function return a Result type, which is an enum that can be either Ok or Err.
// The Ok variant contains a String, which is the username read from the file,
// and the Err variant contains an io::Error, which is the error that occurred while reading the file.
// It's up to the calling code to decide what to do with the Result.
// It coud panic, use a default username, or look up the username somewhere else.
// This is a common pattern in Rust, that's why Rust provides the `?` operator to make it easier to propagate errors.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// This function achieve the same result as the previous one, but using the `?` operator.
// The '?' operator is basically a shorthand for the match statement.
// If the value is an Ok variant, it will return the value inside the Ok variant.
// If the value is an Err variant, it will return the error to the caller as if
// we used the return keyword with the Err variant.
// There is a difference between the match expression and the `?` operator:
// error values that have the '?' operator called on them 
// pass through the from function, defined in the From trait in the standard library,
// which is used to convert types from one to another.
// The error type received is then converted to the error type expected by the function.
// This is usefull when a function returns one error type to represent all the ways a function might fail,
// even if parts might fail for many different reasons.
// NB: The ? operator can only be used in functions whose return type is compatible with the value the ? is used on.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

    // We coud even make this function shorter by chaining methods calls
    //let mut username = String::new();
    //File::open("hello.txt")?.read_to_string(&mut username)?;
    //Ok(username)

    // Or even shorter
    //fs::read_to_string("hello.txt")
}

// The '?' operator can also be used in functions that return a Option type.
// If the value is None, the None will be returned early from the function 
// at that point. If the value is Some, the value inside the Some is 
// the resultant value of the expression, and the function continues.
// Note that you can use the ? operator on a Result in a function that returns Result,
// and you can use the ? operator on an Option in a function that returns Option, 
// but you can’t mix and match.
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// So far, all the main functions we have written have returned ()
// There are restrictions on what is returned on the main.
// Luckily, main can also return a Result<(), E>
// For the moment, let's interpret Box<dyn Error> as "any kind of error".
//fn main() -> Result<(), Box<dyn Error>> {
//    let greeting_file = File::open("hello.txt")?;
//
//    Ok(())
//}
// When a main function returns a Result<(), E>, the executable will exit with a 
// value of 0 if main returns Ok(()) and will exit with a nonzero value if main returns an Err value.
// The main function may return any types that implement the std::process::Termination trait, which contains a function report that returns an ExitCode. 
// See chapter 9.3 to guidelines on when to panic! or return a Result.

// Creating custom types for validation
// Rember the example of Guessing game?
//loop {
//    // --snip--
//
//    let guess: i32 = match guess.trim().parse() {
//        Ok(num) => num,
//        Err(_) => continue,
//    };
//
//    if guess < 1 || guess > 100 {
//        println!("The secret number will be between 1 and 100.");
//        continue;
//    }
//
//    match guess.cmp(&secret_number) {
//        // --snip--
//}

// This is not an ideal solution: if it were absolutely critical that the program only operated on values between 1 and 100, 
// and it had many functions with this requirement, having a check like this in every function would be tedious (and might impact performance).
// we can make a new type in a dedicated module and put the validations in a function to create an 
// instance of the type rather than repeating the validations everywhere. 
// That way, it’s safe for functions to use the new type in their signatures and confidently use the values they receive.
pub struct Guess {
    value: i32,
}

impl Guess {
    // Constructor for Guess
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    // getter method to access the value
    pub fn value(&self) -> i32 {
        self.value
    }
}
