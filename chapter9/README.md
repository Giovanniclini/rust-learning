# Chapter 9 – Error Handling: `panic!`, `Result`, and Recoverable Errors

This folder contains notes and code based on Chapter 9 of *The Rust Programming Language*.

### 📚 Topics Covered

- **Unrecoverable Errors with `panic!`**  
  Halting execution when something goes wrong that cannot be reasonably handled, such as out-of-bounds access.

- **Recoverable Errors with `Result<T, E>`**  
  Representing operations that can fail and handling those failures using pattern matching and combinators like `unwrap`, `expect`, and `match`.

- **Propagating Errors**  
  Using the `?` operator to simplify error propagation in functions that return `Result`.

- **Defining Custom Error Types**  
  Implementing the `Error` trait and using custom types to describe failure modes more clearly and idiomatically.

This chapter teaches you how Rust encourages safe, explicit error handling. You’ll learn how to distinguish between unrecoverable and recoverable errors, write robust APIs, and leverage the type system to catch problems at compile time.
