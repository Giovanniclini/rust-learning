// We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. 
// Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test. 
// In order for integration tests to be executed with cargo test, the unit tests must pass!
// You can always execute integration test specifying the name of the file containing the tests
// $ cargo test --test integration_tests
use adder::add_two_with_internal_adder;

#[test]
fn it_adds_two() {
    let result = add_two_with_internal_adder(2);
    assert_eq!(result, 4);
}

#[test]
fn it_adds_two_with_setup() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}