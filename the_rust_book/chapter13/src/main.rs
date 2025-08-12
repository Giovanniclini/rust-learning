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

    // The way a closure captures and handles values from the environment affects which traits the closure implements, 
    // and traits are how functions and structs can specify what kinds of closures they can use. 
    // Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, 
    // depending on how the closure’s body handles the values:

    // 1. FnOnce applies to closures that can be called once. All closures implement at least this trait because all 
    // closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none 
    // of the other Fn traits, because it can only be called once.
    // 2. FnMut applies to closures that don’t move captured values out of their body, 
    // but that might mutate the captured values. These closures can be called more than once.
    // 3. Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, 
    // as well as closures that capture nothing from their environment. 
    // These closures can be called more than once without mutating their environment, 
    // which is important in cases such as calling a closure multiple times concurrently.

    // ---------------------------

    // Iterators
    // e.g. iter()
    // Iterators are lazy, they have no effect until you call methods that consume the iterator. 
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    // All iterators implement the trait:
    pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
    }
    // This means that with the iterator we also need to implement a type Item
    // and the next method will return an Some(Item) until the object we are iterating is empty. 
    // The Iterator trait has a lot of method you with defult implementstion. You only need to implement the next method. 
    // sum is a consuming adapters: it consumes the iterator and return the sum of the value in the iterator.
    // Iterators adapter are method that return another form of the iterator.
    let v1: Vec<i32> = vec![1, 2, 3];

    // This code produces an error since we are not using the iterator that we get from map
    v1.iter().map(|x| x + 1); // add .collect() to get a new iterator :)

     // ---------------
     // An example of closures and iterators
     fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
    // into_iter() takes ownership of the vector transforming the vector into an iterator
    // filter applies the closure that filters base on size of shoes. (closures that capture the environment)
    // The closure captures the shoe_size parameter from the environment and compares the value with each shoe’s size, keeping only shoes of the size specified.

    // Note that loop and iterators have similar performances!





}
