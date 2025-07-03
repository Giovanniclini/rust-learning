fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.

    // you can create an array specifying the element and the amount of time you want to have that
    // element in the array.
    let a = [0;101];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}