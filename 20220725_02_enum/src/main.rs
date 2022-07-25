enum IpAddrKind {
    V4,
    V6,
}
enum IpAddrKind1 {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    route(IpAddrKind::V4);

    let home = IpAddrKind1::V4(127, 0, 0, 1);
    let loopback = IpAddrKind1::V6(String::from("::1"));

    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);
    m.call();
}
fn route(ip_kind: IpAddrKind) {}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}
