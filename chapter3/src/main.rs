use std::io;

fn main() {

    // Variables and Data Types in Rust
    operations();
    booleans_values();
    tuples_and_arrays();
    panick_index();

    // Functions
    parameters_example_function(5, 'm');
    expressions_and_statements();
    let five = return_five();
    println!("The value of five is: {five}");

    // Control Flow
    if_else_example();
    loop_example();
    while_example();
    for_example();

}

fn while_example() {
    // Rust has a built-in language construct for a sequence of loop, if, break that runs until a condition is no more true.
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_example() {
    // Safer and cleaner than using a while loop.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn loop_example() {
    // Loops are used to repeat a block of code.
    // The loop will continue until it is explicitly broken out of.
    // One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. 
    // You might also need to pass the result of that operation out of the loop to the rest of your code. 
    // To do this, you can add the value you want returned after the break expression you use to stop the loop; 
    // that value will be returned out of the loop so you can use it, as shown here:

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // return counter * 2; is also an option. Break will break current loop, Return will exit the current function.
            break counter * 2; // This will return the value of the loop expression
        }
    };

    println!("The result is: {result}");

    // The break keyword exit the innermost loop. 
    // We can label loops in order to break out of an outer loop.
    // This is useful when we have nested loops and we want to break out of the outer loop.
    // The label is defined with an apostrophe followed by the name of the label.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

}

fn if_else_example() {
    // if-else statements are used to control the flow of the program based on conditions.
    // The condition must evaluate to a boolean value. It do not automatically convert other types to boolean.
    // The condition must be a boolean expression, not a statement.
    // When having multiple conditions, consider using match.
    // Only the first if condition that evaluates to true will be executed.
    
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if expressions can also be used to assign a value to a variable.
    // The if expression must have the same type in all branches.
    let condition = true;
    let number = if condition { 5 } else { 6 };
}

// We can implicitly return the last expression. 
// It is mandatory to declare return type of the function.
// If you want to return a value, you must not end the function with a semicolon. This means end the function with an expression.
fn return_five() -> i32 {
    5
}

fn expressions_and_statements() {
    //Statements are instructions that perform some action and do not return a value.
    //Expressions evaluate to a resultant value. 

    let x = 5; // This is a statement, it does not return a value.
    // The following line will produce a compile-time error, statement DO NOT RETURN VALUES.
    // For example, in Rust, you cannot write x = y = 6; because y = 6 is a statement, not an expression.
    // let y = 6 will not return any value to assign to x.
    //let x = (let y = 6);

    // Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression.
    // Expressions do not end with a semicolon, while statements do.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// It is required to specify type of arguments in Rust.
fn parameters_example_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Convention is to use snake case as the conventional style for function and variable names
// We can define functions AFTER or BEFORE the main function.
fn tuples_and_arrays() {
    // tuples
    // tuples are fixed-size collections of values of different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {y}");

    // accessing tuple elements
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // arrays
    // arrays have - like tuples - a fixed size, but all elements must be of the same type.
    // useful when you want to save elements in the stack, rather than the heap.
    // you can specify the type i32 and the size of the array.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // array can also be defined as:
    // this will produce an array of 5 elements, all initialized to 3
    let a = [3; 5];
}

fn booleans_values() {

    // booleans are a primitive type in Rust
    // they can only be true or false
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("The value of t is: {t}");
    println!("The value of f is: {f}");

}

fn operations() {
    
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // Integer division is defined by the type of the operands
    // cfr. python's // 5 / 3 == 1.6666666666666667
    let truncated = -5 / 3; // Results in -1

    println!("Truncated division: {truncated}");

    // remainder
    let remainder = 43 % 5;

}

fn panick_index() {

    // This function panicks if the index is out of bounds.
    // This is a runtime error, not a compile-time error.
    // Usually, other programming languages let wrong memory addresses be accessed,
    // which can lead to undefined behavior.
    // In Rust, this is not allowed: a check that the index is in bounds is performed at runtime.
    // IMPORTANT CHARACTERISTIC OF RUST: SAFETY.

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
