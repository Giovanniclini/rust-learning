// Ownership is a set of rules that govern how a Rust program manages memory.
// Memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.
// Ownership Rules:
// 1. Each value in Rust has a variable that’s called its “owner”.
// 2. A value can only have one owner at a time.
// 3. When the owner of a value goes out of scope, Rust will automatically clean up the value.

fn main() {
    // What is Ownership?
    variable_scope();
    string_literals();
    drop_memory();
    variables_and_data_interacting();
    stack_only_data();
    ownership_and_functions();
    ownership_and_functions_with_return();

    // References and Borrowing
    reference_example();
    dangling_references();
    // To sum up:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.
    slice_type();
    slice_example();
    string_literals();
    other_kind_of_slices();
}

fn other_kind_of_slices() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn slice_example() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    // At this point word lost the meaning while being still valid. 
    // In this case the compiler compiles without problems. 
    // This is an error prone behaviour and can be fixed with slices!
    
    let sliced_word = first_word_slice(&s);

    // s.clear(); // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable. In order to clear s we need to borrow it as mutable.

    println!("The first word is: {}", sliced_word);
}

fn first_word_slice(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return a slice of the string up to the first space
        }
    }

    &s[..] // if no space is found, return the whole string as a slice
    
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn slice_type() {
    // Internally, the slice data structure stores:
    // 1. A pointer to the data.
    // 2. The length of the slice.
    
    let s = String::from("hello world");

    let hello = &s[..5]; // this is equals to // let hello = &s[0..5];
    let world = &s[6..11];
    let hello_world = &s[..]; // this is equals to // let hello_world = &s[0..11];
    println!("{} {} {}", hello, world, hello_world);
}

fn dangling_references() {
    // Dangling references are references that point to a memory location that has been freed.
    // In Rust the compiler guarantees that references will never be dangling references. 
    let reference_to_nothing = solution_to_dangle();
}

//fn dangle() -> &String {
//    // The function returns a reference to a string that is created inside the function.
//    // When the function ends the string goes out of scope and is dropped.
//    // 
//    let s = String::from("hello");
//    &s // This line will cause an error because s is dropped at the end of the function
//}

fn solution_to_dangle() -> String {
    let s: String = String::from("hello");
    s
}

fn reference_example() {
    // To avoid returning the value of s1 to the function caller
    // We can use references.
    // References allow you to refer to some value without taking ownership of it.
    // The reference is pointing the pointer to the value in memory,
    // so the value is not moved, but borrowed.
    // Because the reference does not own it, the value it points to will not be dropped when the reference stops being used.
    // NB: References are immutable by default, meaning you cannot change the value they point to.

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // Mutable references

    // The string must be mutable in order to have the reference mutable.
    let mut s = String::from("hello");

    // Then we create a mutable reference with &mut.
    change(&mut s);

    // NB: Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. 
    // -> error[E0499]: cannot borrow `s` as mutable more than once at a time
    // This restriction prevents DATA RACES!
    // Data races occur when these three conditions are met:
    // 1. Two or more pointers access the same data at the same time.
    // 2. At least one of the pointers is a mutable reference.
    // 3. There's no mechanism being used to synchronize access to the data.
    // Rust solves data races by refusing to compile code with data races.
    // Data races cause undefined behaviour and can be difficult to diagnose.

    // Rust enforces a similar rule for combining mutable and immutable references.
    // -> error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

    let mut s = String::from("hello");

    // We can define two only-read references
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // But it is a problem if we simultaneously try to create a mutable reference.
    // let r3 = &mut s; // The line generates the error[E0502]

    // NB: Reference scope starts from where it is introduced and continues through the last
    // time it is used... for example:

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
}


// To have mutable references we have to specify the mutable reference in the type accepted by the function.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    // Here we cannot change the value of s, because it is a reference.
    s.len()
}

fn ownership_and_functions_with_return() {
    // In order to keep the ownership in the main function, we can return the ownership of the value from the function 
    // along with values that the functions has to return.
    // This can be quite tedious but can be solved with references.

    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn ownership_and_functions() {
    let s = String::from("hello");  // s comes into scope

    // If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error. 
    // These static checks protect us from mistakes.
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // This line produces an ownership error becouse the value was moved in takes_ownership function.
    //println!("{s}");

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // because i32 implements the Copy trait,
                                    // x does NOT move into the function,
    println!("{}", x);              // so it's okay to use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn stack_only_data() {

    // after the code is executed, x is still valide and wasn't moved into y,
    // contrary to the String example.
    // This types are stored entirely on the stack, so copies of the actual value are quick to make. 
    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, 
    // as integers are. If a type implements the Copy trait, variables that use it do not move, 
    // but rather are trivially copied, making them still valid after assignment to another variable.
    // NB: Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait.
    // In general, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy.
    
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

fn variables_and_data_interacting() {
    // Bind value 5 to variable `x`.
    // Make a copy of `x` and bind it to variable `y`.
    let x = 5;
    let y = x;

    // String is made up of three parts:
    // 1. A pointer to the data (the actual string).
    // 2. The length of the string.
    // 3. The capacity of the string (the amount of space allocated for the string).
    // This group of data is stored on the stack. On the heap, the actual string data is stored.
    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. 
    // We do not copy the data on the heap that the pointer refers to.
    // In other words, we have two Strings that have the pointer field pointing to the same data address in the heap.
    let s1 = String::from("hello");
    let s2 = s1;

    // Earlier, we said that when a variable goes out of scope, 
    // Rust automatically calls the drop function and cleans up the heap memory for that variable. 
    // This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. 
    // This is known as a DOUBLE FREE ERROR (Bug Rust is avoiding!) and is one of the memory safety bugs we mentioned previously. 
    // Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
    // Because Rust also invalidates the first variable, instead of being called a shallow copy (cfr. Python) (opposite to Deep Copy), it’s known as a move. 
    // In this example, we would say that s1 was moved into s2. 
    let s1 = String::from("hello");
    // here s1 is out of scope. The following line generates an ownership error.
    // let s2 = s1;

    // Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

    println!("{s1}, world!");

    // Clone method: deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

fn drop_memory() {
    // When a variable goes out of scope, Rust automatically calls the `drop` function to free the memory.
    // This is done to prevent memory leaks.

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // s goes out of scope and `drop` is called, freeing the memory
}

fn variable_scope() {
    // When s comes into scope, it is valid.
    // It remains valid until it goes out of scope.

    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}

fn string_literals() {
    // Strings can be mutated but literals cannot.
    // The difference is how these two types deal with memory.
    // Strings can change in size and are stored in the heap.

    // We are creating a String type from a string literal ("hello").
    let mut s = String::from("hello"); // s is a mutable String
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print "hello, world!"

    // String literals are stored inside the binary (the executable file) and they are string slices by nature.
    // For this reason, thery are immutable and have fixed size.
    // They differ from `String` type, which is a growable, heap-allocated data structure.
    // They also differ from char type, which represents a single Unicode scalar value.
    let s: &str = "Hello, world!";
}