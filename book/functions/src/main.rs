fn main() {
    // let x = (let y = 6); // expected expression, found `let` statement

    let mut x = 1;
    let y = x = 2;
    println!("{}, {:?}", x, y); // 2, ()

    // i.e.
    // - The statement 'let y = 6' doesn't return a value
    // - The expression 'x = 2' returns the unit type ()
}
