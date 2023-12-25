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

    {
        let s = String::from("hello world");
        let word = first_word(&s);
        println!("The first word in \"{}\" is {}", s, word);

        // The first word in "hello world" is 5 // correct

        // s.clear(); // error!
        // println!("The first word in \"{}\" is {}", s, word);
    }

    {
        let my_string = String::from("hello world");
        let word = awesome_first_word(&my_string[..]);
        let word = awesome_first_word(&my_string); // Automatically convert &String to &str (cf. Deref trait)

        let my_string_literal = "hello world";
        let word = awesome_first_word(&my_string_literal[..]);
        let word = awesome_first_word(my_string_literal);
    }

    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];
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

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn awesome_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
