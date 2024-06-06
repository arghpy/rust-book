fn main() {
    let mut s = String::from("Hello");
    println!("{s}");

    s.push_str(" world!");

    println!("{s}");

    take_ownership(&mut s.clone());

    let (str_final, len) = calculate_len(s);

    println!("{str_final}::{len}");
}

fn take_ownership(some_string: &mut String) {
    some_string.push_str(" this time from the function.");
    println!("{some_string}");
}

fn calculate_len(some_str: String) -> (String, usize) {
    let length = some_str.len();
    (some_str, length)
}
