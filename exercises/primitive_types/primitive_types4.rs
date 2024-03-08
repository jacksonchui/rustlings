// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // Slices are a form of ownership and refer to a contiguous chunk of memory
    // this is a string slice
    let nice_slice = &a[1..a.len()-1];

    // no semicolon needed to note the end of a line in Rust
    assert_eq!([2, 3, 4], nice_slice)
}
