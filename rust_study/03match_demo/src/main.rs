#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    println!("Hello, world!");
    let c = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(c));
    let v = Some(0u8);
    //match必须穷举所有情况，太多使用通配符_
    match v {
        Some(1) => println!("one"),
        _ => {}
    }
    if let Some(3) = v {
        println!("three")
    } else {
        println!("other")
    }
}
fn value_in_cents(coin: Coin) -> u8 {
    //match必须穷举所有情况，太多使用通配符_
    match coin {
        Coin::Penny => {
            println!("1");
            1
        }
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(state) => {
            println!("State {:?}", state);
            5
        }
    }
}