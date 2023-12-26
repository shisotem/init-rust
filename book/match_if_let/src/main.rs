#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // match
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

    // if let
    {
        let some_u8_value = Some(3u8);

        // match some_u8_value {
        //     Some(3) => println!("three"),
        //     _ => (),
        // }

        // [pros] simplicity
        // [cons] no inclusivity check
        if let Some(3) = some_u8_value {
            println!("three");
        }
    }

    {
        // let coin = Coin::Quarter(UsState::Alaska);
        let coin = Coin::Penny;

        // let mut count = 0;
        // match coin {
        //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        //     _ => count += 1,
        // }

        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
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
