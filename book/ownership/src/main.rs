fn main() {
    {
        let _s_stack: &str = "hello";
    }
    {
        let mut _s_heap: String = String::from("hello");
        _s_heap.push_str(", world!");
    } // drop _s_heap

    // ---

    // Copy
    let x = 5;
    let y = x;
    // stack: x, stack: y

    // Move
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // error
    // (stack: s1), stack: s2, heap: "hello"
}
