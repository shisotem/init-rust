use std::io;

const OPTION_C_TO_F: u8 = 0;
const OPTION_F_TO_C: u8 = 1;

fn main() {
    let mut selection = String::new();

    loop {
        display_options();
        selection.clear();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read user selection");

        let selection: u8 = match selection.trim().parse() {
            Ok(num) if num == OPTION_C_TO_F || num == OPTION_F_TO_C => num,
            _ => continue,
        };

        let temperature = get_temperature();
        break;
    }
}

fn display_options() {
    println!("Which conversion?");
    println!("0: °C -> °F");
    println!("1: °F -> °C");
}

fn get_temperature() -> f64 {
    let mut temperature = String::new();

    println!("Enter the temperature to convert:");
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read temperature");

    temperature.trim().parse().expect("Invalid temperature")
}

