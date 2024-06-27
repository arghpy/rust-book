enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("My coin is {:.2} dollars.", (value_in_cents(&Coin::Penny) as f32) / 100_f32);
    println!("My coin is {:.2} dollars.", (value_in_cents(&Coin::Nickel) as f32) / 100_f32);
    println!("My coin is {:.2} dollars.", (value_in_cents(&Coin::Dime) as f32) / 100_f32);
    println!("My coin is {:.2} dollars.", (value_in_cents(&Coin::Quarter) as f32) / 100_f32);

    let mut my_coin = value_in_cents(&Coin::Nickel);
    println!(
        "Current value: {}\nAfter addition: {:?}",
        &my_coin, add_one_dollar(&Some(my_coin))
    );

    match my_coin {
        5 => {
            println!("Lucky coin!");
            my_coin += 10;
            println!("You receive 10 more cents. New value: {}", my_coin)
        }
        _ => (),
    };
}

fn value_in_cents(coin: &Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("You chose a dime!");
            10
        },
        Coin::Quarter => 25,
    }
}

fn add_one_dollar(coin: &Option<i32>) -> Option<i32> {
    match coin {
        None => None,
        Some(value) => Some(value + 1)
    }
}
