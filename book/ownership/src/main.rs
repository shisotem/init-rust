fn main() {
    {
        let _s_stack: &str = "hello";
    }
    {
        let mut _s_heap: String = String::from("hello");
        _s_heap.push_str(", world!");
    } // drop _s_heap
}
