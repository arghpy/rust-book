fn main() {
    let s1 = String::from("Hello world!");
    print_str_properties(&s1);

    let mut s2 = String::new();
    populate_str(&mut s2);
    print_str_properties(&s2);
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn populate_str(str: &mut String) {
    str.push_str("Populated");
}

fn print_str_properties(str: &String) {
    println!("String: {}\nLength: {}", str, calculate_length(&str));
}
