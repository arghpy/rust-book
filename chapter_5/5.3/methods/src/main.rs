#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    is_square: bool,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn change_width(&mut self, new_width: u32) {
        self.width = new_width;

        if self.is_square {
           self.height = new_width; 
        }
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
            is_square: true,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
        is_square: false,
    };
    println!("rect1 is: {:#?}", rect1);
    println!(
        "Area of rect1 is {}\n\n",
        rect1.area()
    );

    rect1.change_width(100);
    println!("rect1 is: {:#?}", rect1);
    println!(
        "Area of rect1 is {}\n\n",
        rect1.area()
    );

    let mut sq1 = Rectangle::square(100);
    println!("sq1 is: {:#?}", sq1);
    println!(
        "Area of sq1 is {}\n\n",
        sq1.area()
    );

    sq1.change_width(5);
    println!("sq1 is: {:#?}", sq1);
    println!(
        "Area of sq1 is {}\n\n",
        sq1.area()
    );
}
