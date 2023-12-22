fn main() {
    {
        let _s_stack: &str = "hello";
    }
    {
        let mut _s_heap: String = String::from("hello");
        _s_heap.push_str(", world!");
    } // Drop _s_heap(in stack) with "hello, world!"(in heap)

    // ---

    // Copy
    let x = 5;
    let y = x;
    // [stack: x, stack: y]

    // Move
    let _s1 = String::from("hello");
    let _s2 = _s1;
    // println!("{}, world!", s1); // error
    // [(stack: s1), stack: s2, heap: "hello"]

    // ---

    // clone -> deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2); // ok

    // ---

    makes_copy(x); // copy
    takes_ownership(s1); // move

    // x: available, s1: unavailable

    // ---

    let greeting = String::from("hello");
    let (returned_greeting, len) = calculate_length(greeting); // move: greeting -> s
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // move: s -> returned_greeting, length -> len
}
