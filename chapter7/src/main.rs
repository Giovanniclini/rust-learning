use crate::garden::vegetables::Asparagus;

// This tells the compiler to include the code in src/garden.rs
pub mod garden;

fn main() {
    // Packages, crates and Modules!
    // As a project grows, you should organize code by splitting it into multiple modules and then multiple files.

    // Rust has a number of features that allow you to manage your code’s organization:
    // Packages: A Cargo feature that lets you build, test, and share crates
    // Crates: A tree of modules that produces a library or executable
    // Modules and use: Let you control the organization, scope, and privacy of paths
    // Paths: A way of naming an item, such as a struct, function, or module

    // Crates
    // A crate is the smallest amount of code that the Rust compiler considers at a time. 
    // Even if you run rustc rather than cargo and pass a single source code file,
    // the compiler considers that file to be a crate. 
    // Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.
    // A crate can be a binary crate or a library crate.
    // Binary crates are programs you can compile to an executable that you can run, 
    // such as a command line program or a server. 
    // Each must have a function called main that defines what happens when the executable runs. 
    // All the crates we’ve created so far have been binary crates.
    // Library crates don’t have a main function, and they don’t compile to an executable. 
    // Instead, they define functionality intended to be shared with multiple projects.
    // Most of the times, when rustaceans talk about crates, they mean library crates.

    // Packages
    // A package is a bundle of one or more crates that provides a set of functionality. 
    // A package contains a Cargo.toml file that describes how to build those crates. 
    // A package can contain zero or more binary crates and exactly one library crate.
    // cargo init or cargo new my-project will create a new package. 
    // After we run cargo new my-project, we use ls to see what Cargo creates. 
    // In the project directory, there’s a Cargo.toml file, giving us a package. 
    // There’s also a src directory that contains main.rs. 
    // Open Cargo.toml in your text editor, and note there’s no mention of src/main.rs. 
    // Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. 
    // Likewise, Cargo knows that if the package directory contains src/lib.rs, 
    // the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. 
    // Cargo passes the crate root files to rustc to build the library or binary.
    // Here, we have a package that only contains src/main.rs, meaning it only contains a binary crate named my-project. 
    // If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, 
    // both with the same name as the package. 
    // A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

    // Modules
    // Before we get to the details of modules and paths, here we provide a quick reference 
    // on how modules, paths, the use keyword, and the pub keyword work in the compiler, 
    // and how most developers organize their code. We’ll be going through examples of each 
    // of these rules throughout this chapter, but this is a great place to refer to as a 
    // reminder of how modules work.

    // Start from the crate root: When compiling a crate, the compiler first looks in the 
    // crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary 
    // crate) for code to compile.

    // Declaring modules: In the crate root file, you can declare new modules; say you declare 
    // a “garden” module with `mod garden;`. The compiler will look for the module’s code 
    // in these places:
    // - Inline, within curly brackets that replace the semicolon following `mod garden`
    // - In the file src/garden.rs
    // - In the file src/garden/mod.rs

    // Declaring submodules: In any file other than the crate root, you can declare submodules. 
    // For example, you might declare `mod vegetables;` in src/garden.rs. The compiler will look 
    // for the submodule’s code within the directory named for the parent module in these places:
    // - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
    // - In the file src/garden/vegetables.rs
    // - In the file src/garden/vegetables/mod.rs

    // Paths to code in modules: Once a module is part of your crate, you can refer to code 
    // in that module from anywhere else in that same crate, as long as the privacy rules allow, 
    // using the path to the code. For example, an `Asparagus` type in the garden::vegetables 
    // module would be found at `crate::garden::vegetables::Asparagus`.

    // Private vs. public: Code within a module is private from its parent modules by default. 
    // To make a module public, declare it with `pub mod` instead of `mod`. To make items within 
    // a public module public as well, use `pub` before their declarations.

    // The use keyword: Within a scope, the `use` keyword creates shortcuts to items to reduce 
    // repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, 
    // you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on 
    // you only need to write `Asparagus` to make use of that type in the scope.

    // The crates directory is chapter7

    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    // See folder restaurant for module and paths usage.

}
