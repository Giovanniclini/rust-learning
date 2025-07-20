// created with 'cargo new adder --lib'

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
}

pub fn greeting(name: &str) -> String {
    //format!("Hello {name}!")
    // Introducing a bug:
    format!("Hello!")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

pub fn add_two_with_internal_adder(a: usize) -> usize {
    internal_adder(a, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    // ------------------------------------------------- How to write tests ------------------------------------------------------------

    // We use super becouse we want that everything is in the outer module is accessible
    // to the tests module in order to perform our tests.
    use super::*;

    #[test]
    // It indicates this is a test function.
    // There might be function not indicated by test in the test module that
    // help executing the tests.
    fn it_works() {
        let result = add(2, 2);
        // The asser_eq! macro provided by the standard library compares two arguments for equality (or inequality for assert_ne!).
        // They also print the two values if the assertion fails. 
        // The assert_ne! macro is most useful for cases when we’re not sure what a value will be, but we know what the value definitely shouldn’t be.
        // Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively. 
        // When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits.
        // All primitive types and most of the standard library types implement these traits. 
        // Because both traits are derivable traits, this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum definition.
        assert_eq!(result, 4);
    }
    // If we launch 'cargo test' we get:
    //running 1 test
    //test tests::it_works ... ok

    //test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    //Doc-tests adder

    //running 0 tests

    //test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    // We can ignore test, filter test or test documentation examples associated to our library. We can also test performance (cfr. 0 measured).

    #[test]
    // Tests are run in different threads. So when the main thread sees that
    // a thread has failed, it marks that test as failed.
    fn it_does_not_work() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // The assert! macro provided by the standard library is usefull when you want to ensuer that some tcondition in a test evaluates to true.
        // We give the assert! macro an argument that evaluates to a Boolean. If the value is true, nothing happens and the test passes.
        // If the value is false, the asser! macro panic! to cause the test to fail.
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // Since the correct result of the function is false, we need to negate it before we pass it to assert!.
        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // We'll just assert that the result contains the input parameter
        // since we know that the requirement has not been refined alrady and the exact text may change.
        // We can add optional arguments after the required arguments that are passed to the format! macro, so we can pass a format string that contains {} placeholders.
        // Custom messages are useful for documenting what an assertion means. When a test fails, you'll have a better idea of what the problem is with the code.
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    // In addition to checking return values, it’s important to check that our code handles error conditions as we expect.
    // We do this by adding the attribute should_panic to our test function. The test passes if the code inside the function panics; 
    // the test fails if the code inside the function doesn’t panic.
    #[test]
    #[should_panic]
    fn guess_panic() {
        Guess::new(200);
    }

    // Tests that use should_panic can be imprecise. A should_panic test would pass even if the test panics for a different reason from the one we were expecting. 
    // To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute. 
    // The test harness will make sure that the failure message contains the provided text.
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn less_than_100() {
        Guess::new(200);
    }

    // We can also write tests that use Result<T, E> instead of panicking when they fail.
    // The it_works function now has the Result<(), String> return type. 
    // In the body of the function, rather than calling the assert_eq! macro, we return Ok(()) when the test passes and an Err with a String inside when the test fails.
    // Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, 
    // which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.
    // You can’t use the #[should_panic] annotation on tests that use Result<T, E>. 
    // To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
    #[test]
    fn it_works_with_result() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // ------------------------------------------------- How to run tests ------------------------------------------------------------

    // cargo test compiles your code in test mode and runs the resultant test binary. 
    // The default behavior of the binary produced by cargo test is to run all the tests in parallel and capture output generated during test runs, 
    // preventing the output from being displayed and making it easier to read the output related to the test results.
    // You can specify command line options to change this default behaviour.
    // Some command line options go to cargo test, and some go to the resultant test binary. 
    // To separate these two types of arguments, you list the arguments that go to cargo test followed by the separator -- and then the ones that go to the test binary.
    // Those options are also documented in the “Tests” section of the the rustc book.
    // $ cargo test -- --test-threads=1
    // We can set the number of threads to use to the test binary. In this case we are telling the program not to use any parallelism.
    // It will take longer time but tests will not interfere with each other if they share state (e.g. They are all writing/reading from the same file).

    // By default, if a test passes, Rust’s test library captures anything printed to standard output. (println! output for example).
    // If we run the following tests, we wil see that only the println! statement of the failed function has been reported in the 
    // test output summary.
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
    // If we want to see printed values for passing tests as well, we can tell rust to do so:
    // $ cargo test -- --show-output

    // Sometimes, running all the tests can take time and we might want to run only a subsets of tests.
    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
    // We can run:
    // $ cargo test one_hundred
    // to run only a single test.
    // To run a subset of tests we can just specify part of a test name, and any test whose name matches that value will be run:
    //$ cargo test add
    // This command will run 
    //test tests::add_three_and_two ... ok
    //test tests::add_two_and_two ... ok

    // Finally we can ignore specific tests with the ignore attribute.
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
    // By controlling which tests run, you can make sure your cargo test results will be returned quickly. 
    // When you’re at a point where it makes sense to check the results of the ignored tests and you have time to wait for the results, 
    // you can run cargo test -- --ignored instead. If you want to run all tests whether they’re ignored or not, you can run cargo test -- --include-ignored.

    // ------------------------------------------------- Test Organization ------------------------------------------------------------
    
    // As mentioned at the start of the chapter, testing is a complex discipline, and different people use different terminology and organization. 
    // The Rust community thinks about tests in terms of two main categories: unit tests and integration tests. 
    // Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. 
    // Integration tests are entirely external to your library and use your code in the same way any other external code would, 
    // using only the public interface and potentially exercising multiple modules per test.
    // Writing both kinds of tests is important to ensure that the pieces of your library are doing what you expect them to, separately and together.

    // UNIT TESTS

    // The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected. 
    // You’ll put unit tests in the src directory in each file with the code that they’re testing. 
    // The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test).
    // The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, 
    // not when you run cargo build. This saves compile time when you only want to build the library and saves space in the resultant compiled artifact because the tests are not included. 
    // You’ll see that because integration tests go in a different directory, they don’t need the #[cfg(test)] annotation. 
    // The attribute cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option.
    // By using the cfg attribute, Cargo compiles our test code only if we actively run the tests with cargo test. This includes any helper functions that might be within this module, 
    // in addition to the functions annotated with #[test].

    // Testing private functions
    // There’s debate within the testing community about whether or not private functions should be tested directly, and other languages make it difficult or impossible to test private functions. 
    // Regardless of which testing ideology you adhere to, Rust’s privacy rules do allow you to test private functions.
    // Internal adder is a private function (not defined with pub) and we can access it in the module test.
    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }

    // INTEGRATION TESTS
    // In Rust, integration tests are entirely external to your library. 
    // They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API. 
    // Their purpose is to test whether many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated, 
    // so test coverage of the integrated code is important as well. To create integration tests, you first need a tests directory.
    // We create a tests directory at the top level of our project directory, next to src. Cargo knows to look for integration test files in this directory. 
    // We can then make as many test files as we want, and Cargo will compile each of the files as an individual crate. ( Remeber: crate are single package of Rust code that can be binary or libraries.)
    // For example:
    // adder
    //├── Cargo.lock
    //├── Cargo.toml
    //├── src
    //│   └── lib.rs
    //└── tests
    //    └── integration_test.rs
    // Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope.

    // Submodule in integration tests

    // Suppose you want to make more files in the tests directory to help organize them. 
    // For example, we can add some code to setup that we want to call from multiple test functions in multiple test files.
    // In order to avoid to create a new file common.rs under the tests folder that can be seen by Rust as test module,
    // we can use the old naming convention of Rust.
    //├── Cargo.lock
    //├── Cargo.toml
    //├── src
    //│   └── lib.rs
    //└── tests
    //    ├── common
    //    │   └── mod.rs
    //    └── integration_test.rs
    // After we’ve created tests/common/mod.rs, we can use it from any of the integration test files as a module. 
    // And mod.rs will not be considered as a test crate like other files in the tests directory.

    // Integration tests for Binary Crates (not libraries)

    // If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, 
    // we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement. 
    // Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

    // This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file. 
    // Using that structure, integration tests can test the library crate with use to make the important functionality available. 
    // If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.

    // So when creating binaries, we need a main file that just calls other code in the library defined in the same binary.
    // The main file do not need to be tested, only the library file will be tested.

}
