enum Option<T> {
    Some(T),
    None,
}

fn main() {
    // Generic Types, Traits and Lifetimes.
    // ---------------------- Generic Types -------------------------
    // Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is generics: abstract stand-ins for concrete types or other properties.
    // We use Generics to create definitions for items like function signatures
    // or structs, which we can then use with many different concrete data types.
    // we’ve already used generics in Chapter 6 with Option<T>, in Chapter 8 with Vec<T> and HashMap<K, V>, and in Chapter 9 with Result<T, E>.
    // When we want to eliminate duplicate code, we follow the following steps:
    // 1. Identify duplicate code.
    // 2. Extract the duplicate code into the body of the function, and specify the inputs and return values of that code in the function signature.
    // 3. Update the two instances of duplicated code to call the function instead.
    // Next, we’ll use these same steps with generics to reduce code duplication.

    // Suppose we have to find the largest element in a list
    // We have two list, one of chars, one of i32.
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    // With generics
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    // We can use generics also in struct in a similar way to functions
    //struct Point<T> {
    //    x: T,
    //    y: T,
    //}
    // Since we defined x and y of the same type T, when we defined a new point
    // x and y have to have the same type.
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };
    // To have different types we could define Point like this:
    //struct Point<T, U> {
    //    x: T,
    //    y: U,
    //}

    // As we did for structs, we can define enums that holds generic types.
    //enum Option<T> {
    //    Some(T),
    //    None,
    //}

    // Same for methods definitions
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // If you write a method within an impl that declares a generic type, 
    // that method will be defined on any instance of the type, no matter what concrete type ends up 
    // substituting for the generic type.
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // We can also specify constraint on generic types when defining methods on the type.
    // Only instances of Point<f32> will have a method distance_from_origin becouse
    // it uses only mathematical operations that are available only for floating-point types.
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5, y: 10 };

    struct Pointv2<X1, Y1> {
        x: X1,
        y: Y1,
    }

    // Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures.
    impl<X1, Y1> Pointv2<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Pointv2<X2, Y2>) -> Pointv2<X1, Y2> {
            Pointv2 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Pointv2 { x: 5, y: 10.4 };
    let p2 = Pointv2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // NB: using generic types won’t make your program run any slower than it would with concrete types.
    // Rust accomplishes this by performing monomorphization of the code using generics at compile time. 
    // Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. 
    // In this process, the compiler does the opposite of the steps we used to create the generic function: 
    // the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

    // ---------------------- Traits -------------------------
    // A trait defines the functionality a particular type has and can share with other types.
    // We can use traits to define shared behavior in an abstract way. 
    // IMPORTANT: We can use trait bounds to specify that a generic type can be any type that has certain behavior.
    // cfr. interfaces
    // Suppose we have multiple structs that hold various amount of text, we want to make a media aggregator library crate
    //  named aggregator that can display summaries of data that might be stored in those structs.
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    // Here we declared a trait using the trait keyword, we can also declare the trait as pub so that creates on this crate can 
    // make use of this trait too.
    // Instead of providing an implementation within curly brackets, we use semicolon.
    // A trait can have multiple methods in its body.
    // Following are some implementation of the Summary trait.
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    // Trait implementation
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    
    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }
    // Trait implementation
    impl Summary for SocialPost {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    // Now users of the crate can call the trait methods on instances of NewsArticle and SocialPost in the same way they call
    // regular methods. 
    // The only difference is that the user must bring the trait into scose as well as the types.
    // use aggregator::{SocialPost, Summary}; // if we define the module aggregator
    // Other crates that depend on the aggregator crate can also bring the Summary trait into scope to implement Summary on their own types. 
    // One restriction to note is that we can implement a trait on a type only if either the trait or the type, or both, are local to our crate, i.e. they are implemented in our crate (not from external library).
    // Sometimes it is usefull to have default behaviour for some or all of the methods in a trait.
    pub trait SummaryDefault {
        // Default implementation can call other methods in the same trait
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }
    pub struct NewsArticleDefault {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    // To use a default implementation to summarize instances of NewsArticle, we specify an empty impl block
    impl SummaryDefault for NewsArticleDefault {
        // using summarize default implemnetation
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    // We can still call summarize on NewsArticleDefault
    let article = NewsArticleDefault {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // Another important use of traits is like PARAMETERS!
    // This function accepts any type that implements the Summary trait!!!
    // The & is necessary becouse usually traits are implemented on struct or objects that work on self and self is a reference.
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound; it looks like this:
    //pub fn notify<T: Summary>(item: &T) {
    //    println!("Breaking news! {}", item.summarize());
    //}
    // For example, we can have two parameters that implement Summary. Doing so with the impl Trait syntax looks like this:
    // pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    // If we want to force both parameters to have the same type, however, we must use a trait bound, like this:
    // pub fn notify<T: Summary>(item1: &T, item2: &T) {
    // We can also specify more than one trait bound.
    // pub fn notify(item: &(impl Summary + Display)) {
    // The + syntax is also valid with trait bounds on generic types:
    pub fn notify<T: Summary + Display>(item: &T) {

    // Rust has also another syntax to defint trait bounds to avoid making the signature function hard to read
    // when there are lots of trait bound information.
    // This:
    //fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    // Becomes:
    //fn some_function<T, U>(t: &T, u: &U) -> i32
    //where
    //    T: Display + Clone,
    //    U: Clone + Debug,
    //{
    // We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait. See function returns_summarizable.

    // Finnaly with Traits we can conditionally implement methods
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    // Every Pair<T> implements new.
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // Only Pairs with T that implements Display and PartialOrd implements cmp_display!
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

}

// We can use any identifier as a type parameter name but we will use T becouse
// by convention, type parameter names in Rust are short and Rust's type-naming convention 
// is CamelCase. Short for type, T is the default choice.
// When we use a parameter in the body of the function, we have to declare the parameter name 
// in the signature so the compiiler knows what that name means. 
// We place type name declarations inside angle brackets <>, between the name of the function and the parameter list.
// We read this definition as: the function largest is generic over some type T. This function has one paramete
// named list, which is a slice of values of type T. The largest function will return a reference to a value of the 
// same type T.
// We have to restrict the type of T to only those that implement PartialOrd, otherwise we get the error 
// "binary operation `>` cannot be applied to type `&T`"
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// The ability to specify a return type only by the trait it implements is especially usefull in the context of closures and iterators.
// NB: You can only use impl Trait only if you are returning a single type. You cannot return (based for example on if-else)
// two different types that implement the same Trait.
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}
