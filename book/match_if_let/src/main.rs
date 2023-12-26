// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

fn main() {
    {
        // let coin = Coin::Quarter(UsState::Alaska);
        // let value = value_in_cents(coin);
        // println!("value: {}", value);
        // // State quarter from Alaska!
        // // value: 25
    }

    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    {
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (), // unit value: nothing will happen
        }
    }
}

// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
