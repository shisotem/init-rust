use std::io;

fn main() {
    let num;
    loop {
        match get_num() {
            Ok(n) => {
                num = n;
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
    }

    let res = fibonacci(num);
    println!("{}", res);
}

fn get_num() -> Result<i32, String> {
    let mut num = String::new();
    match io::stdin().read_line(&mut num) {
        Ok(_) => match num.trim().parse::<i32>() {
            Ok(n) => Ok(n),
            Err(_) => Err(String::from("Input was not a number")),
        },
        Err(_) => Err(String::from("Failed to read user input")),
    }
}

fn fibonacci(num: i32) -> i32 {
    match num {
        0 => 0,
        1 => 1,
        _ => fibonacci(num - 1) + fibonacci(num - 2),
    }
}
