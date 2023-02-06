// pub enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn ips() {
//     let home = IpAddr::V4(String::from(172, 0, 0, 1));

// }

// fn route(ip_kind: IpAddrKind) {

// }
// struct Ipn4Addr {

// }
// struct Ipv6Addr {}

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
#[derive(Debug)]
enum Message {
    Quit, 
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {}

