// TODO: Add the missing type of the argument `num` after the colon `:`.
// The type of function parameters must be annotated
fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
