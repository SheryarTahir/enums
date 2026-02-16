use std::option;

#[derive(Debug)]
enum UsState {
    Albama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}
fn main() {

    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value is {}", value_in_cents(coin));

    println!("Add Result is: {}", add(50, None));

    let dice_roll = 6;
    match dice_roll {
        3 => println!("You got a fancy hat."),
        6 => println!("Your fancy hat is removed."),
        other => println!("Move player {}", other),
    };

}

fn add(num: i32, num2: Option<i32>) -> i32 {
    match num2 {
        Some(i) => num + i,
        None => num,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("This is Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alaska) => {
            println!("Hello from Alaska");
            25
        }
        Coin::Quarter(state) => {
            println!("Got Q of value {:?}", state);
            25

        }
    }
}
 
/* 
fn main() {
    let op = Some(1);
    let sum = 2 + op.unwrap_or(0);
    println!("Sum is: {sum}");
} */
/* 
#[derive(Debug)]
enum IpAddKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main () {
    let home = IpAddKind::V4(127, 0, 0, 1);
    let loopback = IpAddKind::V6(String::from("::1"));
    
    route(home);
    route(loopback);
}

fn route (ip: IpAddKind) {
    println!("Routing request to {:?}", ip);
}*/

/* 
#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,
}

struct IpAdress {
    address: String,
    kind: IpAddKind
}

impl IpAdress {
    fn new(address: &str) -> Self {
        Self { 
            address: address.to_string(),
            kind: IpAddKind::V4 
        }
    }
}
fn main() {
    let google_address = IpAdress::new("1.2.3.4");
    route(google_address);
}

fn route(ip: IpAdress) {
    println!("Routing request to Ip: {} of kind: {:?}", ip.address, ip.kind);
} */
