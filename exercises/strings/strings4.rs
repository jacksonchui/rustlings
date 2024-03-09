// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    // creates owned data from borrowing
    string("rust is fun!".to_owned());
    // doesn't convert into different type?
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    // When ref, it is not amendable to being owned
    string_slice(&String::from("abc")[0..1]);
    // Creates a sliced ref .. use to_string or from to convert
    string_slice("  hello there ".trim());
    // Owns a new bit of memory to replace Mon with Tues
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    // Creates a new String object...not just &'static str
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
