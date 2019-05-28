enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// More concise way 
enum IpAddress {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };


    let improved_home = IpAddress::V4(String::from("127.0.0.1"));
    let improved_loopback = IpAddress::V6(String::from("::1"));
}


fn route(ip_kind: IpAddrKind) {}
