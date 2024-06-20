enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let my_coin = Coin::Dime;
    println!("My coin is {} cents.", value_in_cents(my_coin));
}

fn value_in_cents(coin: Coin) -> u8 {
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
