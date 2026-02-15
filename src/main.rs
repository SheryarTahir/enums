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
