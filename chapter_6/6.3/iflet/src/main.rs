fn main() {
    let config_max = Some(3u8);

    match config_max {
        Some(value) => println!("This is the value: {value}"),
        _ => (),
    }

    if let Some(value) = config_max {
        println!("This is the value from if let: {value}");
    }
}
