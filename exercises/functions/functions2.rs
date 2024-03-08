// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(3);
}

// All parts of function signature should have type declaration
fn call_me(num:u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
