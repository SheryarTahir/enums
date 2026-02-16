fn main() {
    let op = Some(1);
    let sum = 2 + op.unwrap_or(0);
    println!("Sum is: {sum}");
}
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
