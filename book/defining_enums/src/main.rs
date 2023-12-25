enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// struct Ipv4Addr {
//     //
// }

// struct Ipv6Addr {
//     //
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

fn main() {
    {
        let four = IpAddrKind::V4;
        route(four);
    }

    {
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    }
}

fn route(ip_type: IpAddrKind) {}
