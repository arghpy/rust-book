fn main() {
    let s1 = String::from("Hello there big world!");
    let s2 = String::from("Bigboy");

    print_first_word(&s1[..]);
    print_first_word(&s2[..]);

    let a = [1, 2, 3, 4];
    let b = &a[..2];

    assert_eq!([1, 2], b);
}

fn first_word(s: &str) -> &str {
    let str_bytes = s.as_bytes();

    for (i, &item) in str_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}

fn print_first_word(str: &str) {
    let subs = first_word(&str);

    println!("String: {str}\nfirst word: {subs}");
}
