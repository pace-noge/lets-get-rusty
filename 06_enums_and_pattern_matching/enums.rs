#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String)
}

struct IpAddress {
    kind: IpAddrKind,
    address: String,
}


fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let localhost = IpAddress {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    println!("{:?}", home);
}

fn route(ip_kind: IpAddrKind) {}