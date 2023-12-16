fn main() {
    for day in 1..=12 {
        println!("On a {} day Christmas my true love sent to me", day);
        for num in (1..=day).rev() {
            println!("{}", num);
        }
        println!();
    }
}
