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
    let v = vec![1,2,3];

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


    // Strings
    // Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str. 
    // String slices, which are references to some UTF-8 encoded string data stored elsewhere. 
    // The String type, which is provided by Rust’s standard library rather than coded into the core language, 
    // is a growable, mutable, owned, UTF-8 encoded string type. 
    // When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, 
    // not just one of those types. 

    let data = "initial contents";
    // .to_string() method available to any type that implements the Display trait.
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // The method push_str takes a string slice as an argument,
    // becouse we do not want push_str to take ownership of the string,
    // otherwise we would not be able to print s2 later.
    s1.push_str(s2);
    println!("s2 is {s2}");

    // The method push takes a single character as an argument.
    let mut s = String::from("lo");
    let char = 'l';
    s.push(char);
    // Char is a primitive type that implements the Copy traits, 
    // so we can use it after we pushed it to the string.
    println!("char is {char}");

    // To concatenate strings we can also use the + operator.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // fn add(self, s: &str) -> String {
    // we have a reference to a String, while the signature of the method requires a reference to a string slice (&str).
    // This is possible becouse Rust can automatically dereference (deref coercion) the String to a string slice,
    // Note that + will take ownership of the first string, append s2 and return ownership to s3.

    // If we have to combine multiple strings, instead of using + operator, we can use the format! macro.
    // From this:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    // To thi:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // This call do not take ownership of s1, s2 and s3,
    let s = format!("{s1}-{s2}-{s3}");

    // Strings indexing
    // Rust does not support indexing into strings.
    let s1 = String::from("hi");
    // The following line will generate an error:
    //let h = s1[0];
    // A string is a wrapper for a Vec<u8>, which is a vector of bytes,
    // and indexing into a vector of bytes will return a byte, not a character.
    // Rust prevent returning a byte instead of a character,
    // to avoid returning an unexpected value when the string contains a multibyte character.
    // You can think that the length of the next string is 12,
    // but Rust will say that is 24 becouse it takes 2 bytes to represent each cyrillic character.
    let hello = "Здравствуйте";

    // There are 3 relevant ways to look at strings from Rust’s perspective:
    // Bytes, scalar values and grapheme clusters.
    // For example the hindi word: “नमस्ते” 
    // can be represented as 18 bytes: [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // unicode scalar values: ['न', 'म', 'स', '्', 'त', 'े'] (also diacritics that do not make sens on their own)
    // grapheme clusters: ["न", "म", "स्", "ते"] (words)

    // Another reason Rust does not provide indexing into strings is that
    // it can not guarantee O(1) access time.

    // Indexing a string is usually not a good idea becouse you do not know
    // what you could get, but you can still use [] with a range to create a string
    // containing particular bytes:
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // &str that contains 4 bytes.

    // The best way to iterate over a string is to use the chars method
    for c in "Зд".chars() {
        println!("{c}");
    }
    // The bytes method returns an iterator over the bytes of the string.
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // Remember that valid Unicode scalar values may be made up of more than one byte.
    // Check contains and replace for searching in a string and substituting parts of a string with another string.
    

    // Hash Maps
    // The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory.
    // Create and add elements in a hash map:
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    // We can access the elements in a hash map using the get method.
    // The get method returns an Option<&V>, so we can use the match expression to handle the case where the key does not exist.
    // We use copied() to get a Option<i32> instead of Option<&i32>,
    // Then unwrap_or(0) will return 0 if the key does not exist in the hash map.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // We can iterate over a hashmap in a similar way to a vector.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // Remember that For types that implement the Copy trait, like i32, the values are copied into the hash map. 
    // For owned values like String, the values will be moved and the hash map will be the owner of those values,

    // When we want to update an hashmap we can chose if we want to:
    // - Overwrite the value if the key already exists
    // - Add a new key-value pair if the key does not exist
    // - Combine the value with the existing value if the key already exists
    // Override
    scores.insert(String::from("Blue"), 25);
    // Adding if key does not exist
    // The return value of the entry method is an enum called Entry that represents a value that might or might not exist.
    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding 
    // Entry key if that key exists, and if not, it inserts the parameter as the new value for this key and returns a mutable reference to the new value.
    scores.entry(String::from("Blue")).or_insert(50);
    // Combine
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
    // The hasing function used is SipHash which prevents denial-of-service attacks,
    // but it's not the fastest hashing function.

}
