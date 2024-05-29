fn main() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 10 {
            break;
        }
    }

    println!("Result: {counter}");

    // loop labels

    let mut remaining = 10;

    'my_label: loop {
        loop {
            if remaining == 20 {
                break 'my_label;
            }
            remaining += 1;
        }
    }
    println!("Remaining: {remaining}");

    // while
    let mut variable = 0;

    while variable < 10 {
        variable += 1;
    }
    println!("Variable: {variable}");

    // array iteration with while
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < arr.len() {
        println!("arr[{index}] = {}", arr[index]);
        index += 1;
    }

    // array iteration with for
    for element in arr {
        println!("{element}");
    }

    // for through a range
    for number in 1..4 {
        println!("Number: {number}");
    }

    // for through a reversed range
    for number in (1..4).rev() {
        println!("Number: {number}");
    }
}
