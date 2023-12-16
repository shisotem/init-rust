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
}
