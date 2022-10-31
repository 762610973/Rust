#![allow(unused)]

fn main1() {
    enum IpAddrKind {
        V4,
        V6,
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    fn route(ip_type: IpAddrKind) {}
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn main2() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    impl IpAddr {
        fn get_ip_addr(self: &Self) -> IpAddr {
            IpAddr::V4(String::from("嘿嘿"))
        }
    }
    // 将数据附加到枚举的每个成员上
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
    #[derive(Debug)]
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr2::V4(129, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home = {:#?}", home);
}

fn main4() {
    let some_number = Some(5);
    println!("some_number = {:?}", some_number);
    let some_string = Some("a string");
    println!("some_number = {:#?}", some_string);
    let absent_number: Option<String> = None;
    println!("some_number = {:#?}", absent_number);
    let another_number: Option<&str> = Some("str");

    println!(
        "value_in_cents(Coin::Quarter(UsState::Alabama)) = {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
    value_in_cents(Coin::Quarter(UsState::Alaska));
    println!(
        "value_in_cents(Coin::Penny) = {}",
        value_in_cents(Coin::Penny)
    );
    let five = Some(5);
    let six = plus_one(five);
    println!("six = {:?}", six);
    let none = plus_one(None);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state = {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main3() {
    let dice_roll = 9;
    match dice_roll {
        3 => println!("3"),
        _ => println!("_"),
        _ => (), //这种情况下不运行任何值
        other => println!("other"),
    }
}

fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
        //
    }
    println!("some_u8_value = {:?}", some_u8_value);

    // if let写法
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        // 可以增加else,用来处理额外情况
    }
    let mut count = 0;
    match count {
        3 => println!("3"),
        _ => count += 1,
    }
}
