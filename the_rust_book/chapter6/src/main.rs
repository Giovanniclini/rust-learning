enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("{self:#?}");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    // Match is made of arms. 
    // Each arm has two parts: a pattern and some code.
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // The i binds to the value contained in Some (Variant of Option, x)
        Some(i) => Some(i + 1),
    }
}


fn main() {
    // Enums!
    // Enums gives you the possibility to say that a value is one of a possible set of values.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // We can call the function route (whcih specify the type IpAddrKind
    // for the ip_kind parameter) we both IpAddrKind kinds becouse they are the same type!
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // We can attach data to each variant of the enum directly, no need for an extra struct!
    // Note that the name of each enum variant becomes a function that constructs an instance of th enum.
    // That is, IdAddr::V4() is a function call that take a String as parameter and return an IpAddr instance.
    // Furthermore, 
    //enum IpAddr {
    //    V4(u8, u8, u8, u8), // each variant can have different types and amount of associated data!!
    //    V6(String),
    //}
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // We can also define methods on enums like with structs!
    let m = Message::Write(String::from("hello"));
    m.call();

    // Options, a special Enum!
    // Other languages have null values (cfr. Python) that leads to errors and crashes undetected by the compiler.
    // Options forces us to encode all the possibilities, either the value exist or do not exist.
    // Options and its variant are included in the prelude.
    // Some and None are variant of options, in this case we do not neet to explicitly type the type of Option becouse
    // we pass associated data.
    let some_number = Some(5);
    let some_char = Some('e');

    // Here we need to explicitly type the data in Option becouse the compiler is not able to infer the type since no data is passed.
    let absent_number: Option<i32> = None;

    // Using options avoid one of the most common errors: assuming data is not null while it is null. How?
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // This line genetares an error:
    //let sum = x + y;
    // The compiler says: "cannot add `Option<i8>` to `i8`": the error is capture at compile time and not at runtime (cfr. Python: 7 + Null -> Runtime error!)
    // Only when we have an Option<i8> (or whatever type of value we’re working with) 
    // do we have to worry about possibly not having a value, 
    // and the compiler will make sure we handle that case before using the value.

    // Match Control Flow!
    // Match let you compare a value agains a series of patterns and thene xecute code based on which
    // pattern matches.
    // match gives:
    // 1. Expressiveness
    // 2. Grant that all possible cases are handled.
    let coin_found = Coin::Penny;
    println!("Value of the coin found: {} cents", value_in_cents(coin_found));

    // We can also have patterns that bind to values
    // We can use that state inside arm (see values_in_cents())
    let coin_found = Coin::Quarter(UsState::Alabama);
    println!("Value of the coin found: {} cents", value_in_cents(coin_found));

    // A common usage of Match is with Options!
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    // NB: Match patterns in Rust are exhaustive! 
    // So you need to specify all the arms!
    // A function like that: 
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //    match x {
    //        Some(i) => Some(i + 1),
    //    }
    //}
    // will generates a compile-time error since Rust knows that the arm None is missing!

    // If we do not want to explicitly code all the possibilities when using match
    // we can use other or _
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    // other must be put at the end of the branches and will catch all the patterns that
    // are not specified before. 
    // _ => reroll()
    // It does the same thing but we are not using the value rolled.
    // _ => ()
    // In this case we cover all the cases but we do not do anything.

    // Control flow with if let
    // Combine if and let to match values that match one pattern while ignoring the rest.
    // Note that "3u8" means 3 of type u8
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // This is equivalent to:
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // Using if let is a way of using a more clear and simple expression
    // to enforce a match but with only one pattern to match agains all the other.
    // It is a way of renouncing to have the exhaustiveness constraint enforced by match
    // while having a much simpler expression. It is a trade-off the developer has to think about.

    // We can also add a else
    // So this expression:
    let coin = Coin::Quarter(UsState::Alabama);

    let mut count = 0;
    // also match constructs take ownership!
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    // becomes like that
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    // There is another pattern let-else
    // see describe_state_quarter

}

fn route(ip_kind: IpAddrKind) {}
fn remove_fancy_hat() {}
fn add_fancy_hat() {}
fn move_player(other: u8) {}

fn describe_state_quarter_let_else(coin: Coin) -> Option<String> {
    // This version is more clear and easier to read compared
    // to the version with if let.
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter_if_let(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}


