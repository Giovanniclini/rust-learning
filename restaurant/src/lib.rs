// This is a library. 
// Library do not compile to an executable but can be used from other programs
// This library has been created with: 'cargo new --lib restaurant'

// Modules let us organize code within a crate for readability and easy reuse.
// Modules also allow us to control the privacy of items because code within a module is private by default.
// Private items are internal implementation details not available for outside use.
// We can chose tomake modules and items within them public.

// We mentioned that src/main.rs and src/lib.rs are called crate roots. 
// The reason for their name is that the contents of either of these two files 
// form a module named crate at the root of the crate’s module structure, known as the module tree:

//crate
// └── front_of_house
//     ├── hosting
//     │   ├── add_to_waitlist
//     │   └── seat_at_table
//     └── serving
//         ├── take_order
//         ├── serve_order
//         └── take_payment

// Notice that the entire module tree is rooted under the implicit module named crate.

// Items in a parent module can’t use the private items inside child modules, 
// but items in child modules can use the items in their ancestor modules. 
// This is because child modules wrap and hide their implementation details, 
//but the child modules can see the context in which they’re defined. 
// Rust chose to have the module system function this way so that hiding inner implementation details is the default. 
// That way, you know which parts of the inner code you can change without breaking outer code. 
// However, Rust does give you the option to expose inner parts of child modules’ code to outer ancestor modules 
// by using the 'pub' keyword to make an item public.

// To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. 
// To call a function, we need to know its path.
// A path can take two forms:
// 1. An absolute path is the full path starting from a crate root; 
// for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
// 2. A relative path starts from the current module and uses self, super, or an identifier in the current module.
// Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

// With front of the house usually we mean the part of the restaurant where customers are served,
// while the back of the house is where food is prepared and other behind-the-scenes work happens.
// Modules are defined with the `mod` keyword.
// Inside modules you can find other modules too!
mod front_of_house {
    // If we only specify the pub keyword here,
    // if we can access the module front of the house we can access hosting. 
    // But the contents of hosting are still private.
    // Making the module public, doesn't make the contents public.
    // The pub keyword on a module only lets code in its ancestor modules refer to it,
    // not access its inner code. 
    pub mod hosting {
        // we need to make also the function public
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// In the absolute path, we start with crate, the root of our crate’s module tree. 
// The front_of_house module is defined in the crate root. While front_of_house isn’t public, 
// because the eat_at_restaurant function is defined in the same module as front_of_house 
// (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant. 
// Next is the hosting module marked with pub. We can access the parent module of hosting, so we can access hosting. 
// Finally, the add_to_waitlist function is marked with pub and we can access its parent module, so this function call works!

// See The Rust API Guidelines to know more about how to share your library so other projects con use your code.

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Best practices for packages with a binary and library
// We mentioned that a package can contain both a src/main.rs binary crate root as well as a src/lib.rs library crate root, 
// and both crates will have the package name by default. 
// Typically, packages with this pattern of containing both a library and a binary crate will have 
// just enough code in the binary crate to start an executable that calls code within the library crate. 
// That is, in the binary crate will only be the structure of the program, and the actual functionality will be in the library crate.
// This lets other projects benefit from most of the functionality that the package provides because the library crate’s code can be shared.
// The module tree should be defined in src/lib.rs. Then, any public items can be used in the binary crate by starting paths 
// with the name of the package. The binary crate becomes a user of the library crate just like a completely external 
// crate would use the library crate: it can only use the public API. T
// his helps you design a good API; not only are you the author, you’re also a client!
// See chapter 12 of the rustbook for a practical example of this pattern.

// We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using super at the start of the path.
// It is like using `..` in a filesystem path to go up one level.

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    // We can use the pub keyword to make a struct public.
    // If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. 
    // We can make each field public or not on a case-by-case basis.

    // Also, note that because back_of_house::Breakfast has a private field, 
    // the struct needs to provide a public associated function that constructs an instance of Breakfast (we’ve named it summer here). 
    // If Breakfast didn’t have such a function, we couldn’t create an instance of Breakfast in eat_at_restaurant because we couldn’t set 
    // the value of the private seasonal_fruit field in eat_at_restaurant.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // In contrast, if we make an enum public, all of its variants are public as well.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_struct_enum() {
    // Struct
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    // Enum
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// We can also use the keyword 'use' to bring paths into scope.
// It is like a shortcut for the full path.

use crate::front_of_house::hosting;

// Furthermore, with re-exporting we can make items available at a higher level in the module tree.
// External users of our library can use the re-exported items without needing to know the full path.
pub use crate::front_of_house::hosting;
// Without 'pub use' external code wouldve needed to use the full path to access the hosting module.
// and also front_of_the_house would have been required to be declared as pub.
// Like this we have re-exported the hosting module, so from an external crate,
// it would be accessible with
// restaurant::hosting::add_to_waitlist();
// Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. 

pub fn eat_at_restaurant_use_keyword() {
    hosting::add_to_waitlist();
}

// hosting will not be accessible if outside the particular scope it is defined.
// The following code will generate an error:
//mod customer {
//    pub fn eat_at_restaurant() {
//        hosting::add_to_waitlist();
//    }
//}

// Note that we coud've also bring into scope only the function we needed:
// use crate::front_of_house::hosting::add_to_waitlist;
// This would have the same effect as the previous example, but it would not IDIOMATIC.
// Specifying the parent module when calling the function makes it clear 
// that the function isn’t locally defined while still minimizing repetition of the full path.
// On the other hand, struct and enums are brought into scope with the full path. 

// We can also use the keyword 'as' to rename items when bringing them into scope if we find in the situation 
// where we have two items with the same name in scope.
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

// External packages
// To use external packages we need to add them to the Cargo.toml file and to import them in the code.
// For example, to use the rand crate, we add the following line to Cargo.toml:
// [dependencies]
// rand = "0.8.5"
// Then we can use it in our code like this:
// use rand::Rng;
// For the standard library, we do not need to add it to Cargo.toml.
// We just need to use it in our code.
// use std::collections::HashMap;

// Nested Paths
// We can use nested paths to bring multiple items into scope from the same module.
// This:
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
// Can be written as this:
// --snip--
use std::{cmp::Ordering, io};
// --snip--
// Or even this:
use std::io;
use std::io::Write;
// Into:
use std::io::{self, Write};

// The Glob operator
// If we want to bring all public items from a module into scope, we can use the glob operator *.
use std::collections::*;
// This use statement brings all public items defined in std::collections into the current scope. 

// To defines modules in different files 
// we can use the mod keyword in the crate root file (src/lib.rs or src/main.rs)
// The compiler will look for the module’s code in these places:
// - Inline, within curly brackets that replace the semicolon following `mod module_name`
// - In the file src/module_name.rs
// - In the file src/module_name/mod.rs
// If we want to define a submodule, we can use the same approach.
// For example, if we want to define a submodule named vegetables in the garden module,
// we can use the following code in src/garden.rs:
// mod vegetables;
// Just put in the other file the code you would have put in the curly brackets.
// The compiler will look for the submodule’s code in these places:
// - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
// - In the file src/garden/vegetables.rs
// - In the file src/garden/vegetables/mod.rs
// This way we can organize our code in a modular way, making it easier to read and maintain.
// The mod approach is the old way of organizing code in Rust.