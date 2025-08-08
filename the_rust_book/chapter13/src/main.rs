fn main() {
    // Closures
    // Functions stored in variables that can be represented in different ways
    //fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    // --------------------------

    // We get an error since the string type get locked into the closuer
    // We get an error when we try to call the same closure on a different type. 
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);

    // ---------------------------

    // Another interesting example.
    // Closures can borrow mutably immutably or move values based on what the closures does.
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // In this case we are mutably borrowing list in the definition of the closure. 
    // We are then creating a mutable reference. 
    let mut borrows_mutably = || list.push(7);

    // If we try to println! the list here we would get an error becose
    // the mutable reference of the list created in the definition of the closure is alive 
    // until the call to the closure and we know we cannot have at the same time
    // a mutable reference an immutable reference (created by the println!).

    borrows_mutably();
    println!("After calling closure: {list:?}");

    // If we want to force move a value we can use the move keyword in this way:
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

}
