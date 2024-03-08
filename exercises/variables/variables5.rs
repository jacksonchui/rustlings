// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    { 
        // Shadowing is cool, because just like in shell or a crazy
        // version of scope in cpp, it allows us to access local vars
        // inside a scope.
        let number = 3; // don't rename this variable
        println!("Number plus two is : {}", number + 2);
    }
}
