#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    value: String,
}

impl IpAddr {
    fn display(&self) {
        println!(
            "Type: {:?}\nValue: {}\n",
            self.kind, self.value
        )
    } 
}

#[derive(Debug)]
enum IpAddrConcise {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddrValues {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // The body of the method would use self to get the value that we called the method on
    }
}

fn main() {
    let router = IpAddr {
        kind: IpAddrKind::V4,
        value: String::from("192.168.1.1"),
    };

    let localhost = IpAddr {
        kind: IpAddrKind::V6,
        value: String::from("::1"),
    };

    router.display();
    localhost.display();

    let router = IpAddrConcise::V4(String::from("192.168.1.1"));
    let localhost = IpAddrConcise::V6(String::from("::1"));

    println!("router: {:?}", router);
    println!("localhost: {:?}", localhost);

    let router = IpAddrValues::V4(192, 168, 1, 1);
    let localhost = IpAddrConcise::V6(String::from("::1"));

    println!("router: {:?}", router);
    println!("localhost: {:?}", localhost);

    let quit = Message::Quit;
    let point = Message::Move { x: 5, y: 10 };
    let idk = Message::Write(String::from("Hello"));
    let new_color = Message::ChangeColor(170, 50, 200);

    idk.call();
}
