fn main() {
    let ordinal = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let characters = [
        "a partridge in a pear tree.",
        "two turtle doves,",
        "three French hens,",
        "four calling birds,",
        "five golden rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,",
    ];

    for day in 1..=12 {
        println!(
            "On a {} day Christmas my true love sent to me",
            ordinal[day - 1]
        );
        for num in (1..=day).rev() {
            if day != 1 && num == 1 {
                print!("and ")
            }
            println!("{}", characters[num - 1]);
        }
        println!();
    }
}
