
// Similar to Tuples, 
// You do not need to rely on the order of data to access the values stored.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }

    // We can also pass other parameters to methods!
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {

    // Structs are Rust "classes".

    // To use a struct we create an instance of the struct like this:
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // If the struct is mutable we can change a value by using the dot notation (same for reading)
    user1.email = String::from("mariorossi@gmail.com");

    let user2 = build_user(String::from("pippopluto@gmail.com"), String::from("pippopluto99"));

    // Struct update syntax: we can all values from user1 and we update only the email.
    let user3 = User {
        email: String::from("emaildiprova@mail.com"),
        ..user1
    };
    //NB: After creating user3 we can no longer use user1 variable!!!
    // Why? Becouse String in the username of user1 was MOVED into user3!
    // If we had given user3 new value for both email and username, then user1
    // woudl still be valid becouse active (bool) and sign_in_count (int) are types
    // that implement the copy trait!!

    // Rust also implement tuple structs!
    // Note that each struct we define has its own type even though the fields within the struct might have
    // the same types!
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Access a value
    let red_in_black = black.0;
    // Destructure: you still need to specify the type
    let Point(x, y, z) = origin;

    // Rust also have unit-like structs
    // useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
    let subject = AlwaysEqual;

    // If we use &str we need to specify lifetimes!    
    //let user1 = User {
    //    active: true,
    //    username: "someusername123",
    //    email: "someone@example.com",
    //    sign_in_count: 1,
    //};

    // Adding useful informations with derivedd traits
    let scale = 2;
    // Struct do not implement the trait Display (formatting output for direct user usage)
    // differently from primitive types that instead implement it.
    // if we do println!("rect1 is {rect1:?}"); we can tell Rust to use the trait Debug formatting.
    // Rust does include functionality to print out debugging information, 
    // but we have to explicitly opt in to make that functionality available for our struct. 
    // To do that, we add the outer attribute #[derive(Debug)]
    // #[derive(Debug)]
    // struct Rectangle { ... }
    let rect1 = Rectangle {
        // To print value we can also use the dbg! macro
        // which take ownership, print file and line number of whare the macro occurs 
        // and the resultant value of that expression
        // Note that dbg! macro - differently from println! macro - points to standard error console stream.
        // Here the ownership is returned to rect1.width
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    // for pretty-print
    println!("rect1 is {rect1:#?}");

    // Here we use a reference becouse we do not want dbg! to take ownership of rect1
    // The output uses the pretty debug formatting of the Rectangle type.
    dbg!(&rect1);

    // Rust make available several traits to use with #derive attribute (see Rectangle struct).

    // Methods are similar to functions: we declare them with the fn keyword and a name, 
    // they can have parameters and a return value, and they contain some code that’s run when 
    // the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct 
    // (or an enum or a trait object), and their first parameter is always self, 
    // which represents the instance of the struct the method is being called on.

    // Methods must have a parameter named self of type Self for their first parameter, so Rust let us abbreviate
    // this with only the name self. 
    // Note the method borrows the Self instance. 
    // They can also borrow self mutably with &mut self.
    // Using only self is rare but possible, when the method transforms self into something else and you want 
    // to prevent the caller from using the original instance after the transformation.
    //impl Rectangle { // Everything within the implementing block will be associated with the Rectangle type.
    //    fn area(&self) -> u32 { // &self is short for self: &Self 
    //        self.width * self.height
    //    }
    // We can also implements methods that have the same name of one of the struct's fields.
    //fn width(&self) -> bool {
    //    self.width > 0
    //}
    // Usually, we implement this technique to implement a getter so the field can stay private. 
    //}
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // Rust can detect if the method is reading (&self), mutating (&mut self), or consuming (self)
    // The following lines are the same!
    //p1.distance(&p2);
    //(&p1).distance(&p2);

    // Associated functions
    // All functions defined within an impl block are called associated functions 
    // because they’re associated with the type named after the impl. 
    // We can define associated functions that don’t have self as their first parameter 
    // (and thus are not methods) because they don’t need an instance of the type to work with.
    // NB: Associated functions are often use like constructors to return a new instance of that struct.
    // The Self keywords in the return type and in the body of the function 
    // are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
    // To call this associated function, we use the :: syntax
    //impl Rectangle {
    //    fn square(size: u32) -> Self {
    //        Self {
    //            width: size,
    //            height: size,
    //        }
    //    }
    //}

    // Also note that it is possible to define multiple imp. blocks! and it is equivalent as putting everything in the same imp. block.

}

fn build_user(email: String, username: String) -> User {

    
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
    // Shorthand for:
    //User {
    //    active: true,
    //    username: username,
    //    email: email,
    //    sign_in_count: 1,
    //}
}
