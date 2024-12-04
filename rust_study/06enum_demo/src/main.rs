enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {}
}
enum IpAddrKind {
    V4,
    V6,
}
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn main() {
    println!("Hello, world!");
    let home = IpAddrKind2::V4(127, 0, 0, 1);
    let loopback = IpAddrKind2::V6(String::from("::1"));
    two();
    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 13 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(0, 2, 2);
    m.call();
}
fn first() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V4);
}
fn route(ip_kind: IpAddrKind) {}
fn two() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}