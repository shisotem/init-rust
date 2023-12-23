fn main() {
    {
        let mut s = String::from("hello world");
        let length = length_of_first_word(&s);
        println!("The length of the first word in \"{}\" is {}", s, length);

        // The length of the first word in "hello world" is 5 // correct

        s.clear();
        println!("The length of the first word in \"{}\" is {}", s, length);

        // The length of the first word in "" is 5 // incorrect
    }
}

// problem: s and length are not linked
// => solution: Synchronize using SLICE!
fn length_of_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
