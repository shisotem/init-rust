fn main() {
    {
        let _s_stack: &str = "hello";
    }
    {
        let mut _s_heap: String = String::from("hello");
        _s_heap.push_str(", world!");
    } // drop _s_heap

    // ---

    // -> Stack (x: 5, y: 5)
    let x = 5;
    let y = x;

    // Move
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // error
}
