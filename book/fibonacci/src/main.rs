use std::io;

fn main() {
    let num = get_num();
    println!("{num}");
}

fn get_num() -> i32 {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Fail to read user input");
    num.trim().parse().expect("Not a number")
}
