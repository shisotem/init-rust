use std::io;

fn main() {
    let fibonacci_index;
    loop {
        match get_index() {
            Ok(n) => {
                fibonacci_index = n;
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
    }

    let fibonacci_result = fibonacci(fibonacci_index);
    println!("{}", fibonacci_result);
}

fn get_index() -> Result<i32, String> {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => match user_input.trim().parse::<i32>() {
            Ok(n) => Ok(n),
            Err(_) => Err(String::from("Input was not a number")),
        },
        Err(_) => Err(String::from("Failed to read user input")),
    }
}

fn fibonacci(index: i32) -> i32 {
    match index {
        0 => 0,
        1 => 1,
        _ => fibonacci(index - 1) + fibonacci(index - 2),
    }
}
