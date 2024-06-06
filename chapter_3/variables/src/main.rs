// Constants
const MY_VAR: u32 = 15;

fn main() {
    // Shadowing
    let x = 5;
    println!("x is {x}");

    let x = x + 1;
    println!("x is {x}");

    {
        let x = x * 2;
        println!("x is {x}");
    }

    println!("x is {x}");

    println!("MY_VAR is {MY_VAR}");

    // Tuples - won't change in size
    let (x, y, z) = (1, 3.4, 'c');

    print!("x = {x}, y = {y}, z = {z}\n");

    let tuple = (5, 10.9, "idk");

    let (a, b, c) = tuple;

    print!("a = {a}, b = {b}, c = {c}\n");
    print!("tuple.0 = {}, tuple.1 = {}, tuple.2 = {}\n", tuple.0, tuple.1, tuple.2);

    // Arrays - won't change in size
    let arr: [u32; 5] = [1, 2, 3, 4, 5];

    for i in 0..arr.len() {
       println!("arr[{i}] = {}", arr[i]);
    }

    // pre-initialization
    let empty_arr = [0; 10];

    for i in 0..empty_arr.len() {
       println!("empty_arr[{i}] = {}", empty_arr[i]);
    }
}
