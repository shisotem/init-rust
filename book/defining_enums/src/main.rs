enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    {
        let four = IpAddrKind::V4;
        route(four);
    }
}

fn route(ip_type: IpAddrKind) {}
