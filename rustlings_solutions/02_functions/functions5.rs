// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    // I had to remove semicolon to implicitly return the result.
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}