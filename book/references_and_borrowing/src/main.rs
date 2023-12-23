fn main() {
    {
        let s1 = String::from("hello"); // [1]: s1(stack) -> "hello"(heap)
        let len = calculated_length(&s1);
        println!("The length of '{}' is {}.", s1, len); // [3]: s1(stack) -> "hello"(heap)
    } // s1 scope out, drop s1 and "hello"

    {
        let mut s = String::from("hello");
        change(&mut s);
    }

    {
        // let mut s = String::from("hello");
        // let r1 = &mut s; // OK
        // let r2 = &mut s; // NG
    }

    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
        }
        let r2 = &mut s;
    }

    {
        // let mut s = String::from("hello");
        // let r1 = &s; // OK
        // let r2 = &s; // OK
        // let r3 = &mut s; // NG
    }
}

fn calculated_length(s: &String) -> usize {
    // [2]: s(stack) -> s1(stack) -> "hello"(heap)
    s.len()
} // s scope out, but not drop s1 and "hello"

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
