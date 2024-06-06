fn main() {
    let number = 3;

    if number < 5 {
        println!("Lesser");
    } else if number == 5 {
        println!("Equal");
    } else {
        println!("Greater");
    }

    let other_num = if number < 5 { number } else { 5 };

    println!("other_num: {other_num}");
}
