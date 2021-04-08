enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKind2 {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2{
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddrKind2::V4(String::from("127.0.0"));

}
