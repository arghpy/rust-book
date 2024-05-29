// use std::u32;
// use std::io;
//
// const IDK: u32 = 10;

fn main() {
    // let x = 5;
    // // println!("The value of x is: {x}");
    // let x = x + 1;
    // // println!("The value of x is: {x}");
    // // println!("The value of IDK is: {IDK}");
    // {
    //     let x = x * 2;
    //     // println!("The value of x in inner scope is: {x}");
    // }
    // // println!("The value of x is: {x}");
    //
    //
    // let guess: u32 = "42".parse()
    //     .expect("Not a number");
    //
    // // println!("Guess from string to integer: {guess}");
    //
    //
    // let tup = (5.43, 128, 5, 'c');
    // let (x, _y, _z, _a) = tup;
    // let integer = tup.1;
    // // println!("The tuple: {x}");
    // // println!("The integer: {integer}");
    //
    // let arr = [1, 2, 3, 4, 5];
    // let second_el = arr[1];
    //
    // let another_arr: [u32; 3] = [1, 2, 3];
    // let first_el: u32 = another_arr[0];
    //
    // // println!("The second el of arr: {second_el}");
    // // println!("The first el of another_arr: {first_el}");
    //
    //
    // // Make an array of 5 elements, all being 0s
    // let zero_arr = [0; 5];
    //
    // println!("The elements of the array are:");
    // for el in 0..arr.len() {
    //     let element = arr[el];
    //     println!("arr[{el}] = {element}");
    // }

    // loop {
    //     println!("Choose an index to show the element:");
    //
    //     let mut index = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut index)
    //         .expect("Failed to read line.");
    //
    //     let index: usize = index
    //         .trim()
    //         .parse()
    //         .expect("Failed to parse string to number.");
    //
    //     let element = arr[index];
    //
    //     println!("Read index {index}. The element is {element}.");
    // }

    another_func();
}

fn another_func() {
    println!("Hello from other function!");
}
