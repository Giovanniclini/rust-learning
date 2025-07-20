# Chapter 11 – Writing Automated Tests

This folder contains notes and code based on Chapter 11 of *The Rust Programming Language*.

### 📚 Topics Covered

- **Writing Tests with `#[test]`**  
  Creating unit tests using Rust’s built-in test framework by annotating functions with `#[test]`.

- **Using `assert!`, `assert_eq!`, and `assert_ne!`**  
  Common macros to verify conditions and compare values in test cases.

- **Testing with `Result<T, E>`**  
  Writing test functions that return `Result` for more expressive error reporting without panics.

- **Organizing Tests**  
  Structuring tests within modules, using `#[cfg(test)]` to separate test code from production code.

- **Integration Tests**  
  Placing tests in the `tests/` directory to validate public API behavior from the outside.

This chapter teaches you how to write reliable, automated tests in Rust. You'll learn how to verify correctness, structure test code clearly, and ensure your programs behave as expected during development and after changes.
