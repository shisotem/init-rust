fn main() {
    // // error
    // let number = 3;
    // if number {
    //     println!("number was three");
    // }

    let number = 3;
    if number != 0 {
        println!("number was something other than zero"); // arm
    } else {
        println!("number was zero"); // arm
    }

    // ---

    // error
    // let condition = true;
    // let number = if condition { 5 } else { "six" };

    // ok
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // ---

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // ---
}
