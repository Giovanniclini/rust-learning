fn main() {
    // Rust’s standard library includes a number of very useful data structures called collections.
    // Collections can contain multiple values.
    // Data these collections store is stored in the heap (differently from standard array and tuples).
    // Amount of data do not need to be known at compile time.
    // 
    // Vectors : allows to store a variable number of values next to each other.
    // We are specifying i32 as values contained in the vector.
    // with Vec<T> we can say that any value can be stored in the vector.
    let v: Vec<i32> = Vec::new();

    // The macro vec! will create a new vector that holds the values you give it
    let v = vec![1,2,3]

    // Adding elements to the vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Two ways of reference a value stored in a vector: 
    // 1) indexing
    // We get the reference to the element at the index
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // 2) Get method
    // We get Option<&T> that we can use with match.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // Referencing an element outside of vector size
    // This line panicks!
    //let does_not_exist = &v[100];
    // This return None
    let does_not_exist = v.get(100);


    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // We can not modify the initial vector if we borrowed a value of the vector as mutable!!!
    //v.push(6); // Error!!

    // In this way we prevent a bad situation: if we need to reallocate memory to add an element 
    // becouse the size of the vector it is not enough the reference to the first elmeent would be 
    // not valid!!

    println!("The first element is: {first}");

    // Iterating and modifying a vector.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Vectors can only stores a single type....
    // Fortunately, the variants of an enum are defined under the same enum type. 
    // So when we need one type to represent elements of different types, we can define and use and enum!
    // Rust need exactly to know how much memory on the heap will be needed to store each element.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // When a vector goes out of scope, it is dropped.
}
