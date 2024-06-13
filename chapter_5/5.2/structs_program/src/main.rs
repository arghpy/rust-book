#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Straight forware (Simple) Approach
    let width = 32;
    let height = 50;

    println!(
        "The area of the rectangle in pixels is {}",
        area_simple(width, height)
    );

    // Using tuples
    let rect1 = (32, 50);

    println!(
        "The area of the rectangle in pixels is {}",
        area_tuple(rect1)
    );

    // Using a struct
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(32 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle in pixels is {}",
        area_struct(&rect2) // We borrow the structure in order for it to mantain ownership in main
    );

    println!("The regtangle struct: {:#?}", rect2);
    dbg!(&rect2);
    println!("The regtangle struct: {:#?}", rect2);
}

fn area_simple(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
