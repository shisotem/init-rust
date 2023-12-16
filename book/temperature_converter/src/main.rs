use std::io;

fn main() {
    let mut selection = String::new();

    loop {
        display_options();
        selection.clear();

        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        match parse_selection(&selection) {
            Some(num) if num == 0 || num == 1 => break,
            _ => continue,
        }
    }

    // println!("{}", selection);
}

fn display_options() {
    println!("Which conversion?");
    println!("0: °C -> °F");
    println!("1: °F -> °C");
}

fn parse_selection(selection: &str) -> Option<i32> {
    selection.trim().parse().ok()
}
