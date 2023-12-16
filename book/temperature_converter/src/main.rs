use std::io;

const OPTION_C_TO_F: i32 = 0;
const OPTION_F_TO_C: i32 = 1;

fn main() {
    let mut selection = String::new();

    loop {
        display_options();
        selection.clear();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read user selection");

        match parse_selection(&selection) {
            Some(num) if num == OPTION_C_TO_F || num == OPTION_F_TO_C => break,
            _ => continue,
        }
    }
}

fn display_options() {
    println!("Which conversion?");
    println!("0: °C -> °F");
    println!("1: °F -> °C");
}

fn parse_selection(selection: &str) -> Option<i32> {
    selection.trim().parse().ok()
}
