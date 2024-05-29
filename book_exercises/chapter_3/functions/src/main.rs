use std::io;

fn raise_to_power(x: usize, power: i32) {
    let mut result: usize = 1;
    for _ in 1..(power + 1) {
       result = result * x; 
    }
    println!("{x} to the power of {power}: {result}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Raise number to the power.");

    let mut number = String::new();
    let mut power = String::new();

    println!("Enter the number:");
    io::stdin() 
        .read_line(&mut number)
        .expect("Failed to read.");
    let number = number
        .trim()
        .parse()
        .expect("Couldn't convert to integer.");

    println!("Enter the power:");
    io::stdin()    
        .read_line(&mut power)
        .expect("Failed to read.");
    let power = power
        .trim()
        .parse()
        .expect("Couldn't convert to integer.");

    raise_to_power(number, power);
    
    let x = plus_one(five());

    println!("X is {x}");
}
