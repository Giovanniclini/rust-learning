// TODO: Fix the compiler error in the function without adding any new line.
// When the function get the ownership of vec, a new binding is created!
// This means that we can add mut to function signature and this will automatically change
// the nature of our parameter!
// Even if the initial variable is not mutable! 
// Remember, we are changing ownership, not borrowing the variable :)
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
