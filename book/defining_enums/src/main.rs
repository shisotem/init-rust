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

// struct QuitMessage; // unit struct
// struct MoveMessage { // struct
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// enum Option<T> {
//     Some(T),
//     None,
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

    {
        let quit_message = Message::Quit;
        let move_message = Message::Move { x: 1, y: 2 };
        let write_message = Message::Write(String::from("hello"));
        let change_color_message = Message::ChangeColor(1, 2, 3);
    }

    {
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None; // type annotation is required

        // let x: i8 = 5;
        // let y: Option<i8> = Some(5);
        // let sum = x + y; // error
    }
}

fn route(ip_type: IpAddrKind) {}
