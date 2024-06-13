struct Student {
    name: String,
    age: i32,
    enrolled: bool,
}

struct Color(i32, i32, i32);
struct Point(f32, f32, f32);

struct AlwaysEqual; // This will be useful when building traits

fn main() {
    let student1 = Student {
        name: String::from("Alex"),
        age: 32,
        enrolled: true,
    };

    let mut student2 = Student {
        ..student1
    };

    student2.name = String::from("Ana");

    let student3 = build_user(String::from("Andrei"), 15);
    println!("Student:\n\tname: {}\n\tage: {}\n\tenrolled: {}", student3.name, student3.age, student3.enrolled);

    let white = Color(0, 0, 0);
    let first_point = Point(1.32, 4.5, 20.3);

    println!("Point:\n\tx: {}\n\ty: {}\n\tz: {}", first_point.0, first_point.1, first_point.2);
    println!("Color: {} {} {}", white.0, white.1, white.2);

    let _subject = AlwaysEqual;
}

fn build_user(name: String, age: i32) -> Student {
    Student {
        name,
        age,
        enrolled: true,
    }
}
