fn main() {
    let mut s = String::from("Hello");
    println!("String: {s}");

    s.push_str(" world!");

    println!("String: {s}");

    take_ownership(s.clone());

    println!("String: {s}");
}

fn take_ownership(some_string: String) {
    println!("String outputed from function: {some_string}");
}
