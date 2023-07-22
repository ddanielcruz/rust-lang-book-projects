#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

impl IpAddrKind {
    fn dial(&self) {
        println!("Dialing: {:?}", self)
    }
}

#[derive(Debug)]
#[allow(unused)]
enum USState {
    Alabama,
    California,
    NewYork
}

#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState)
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }
}

fn main() {
    route(IpAddrKind::V4(String::from("127.0.0.1")));
    route(IpAddrKind::V6(String::from("::1")));
    println!("Coin: {}", Coin::Dime.value_in_cents());
    println!("Coin: {}", Coin::Quarter(USState::California).value_in_cents());
    options();
}

fn route(ip_kind: IpAddrKind) {
    ip_kind.dial()
}

fn options() {
    let some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;

    match some_number {
        Some(value) => println!("Value with match is {}", value),
        _ => ()
    };

    if let Some(value) = some_number {
        println!("Value without match is {}", value);
    }
}
