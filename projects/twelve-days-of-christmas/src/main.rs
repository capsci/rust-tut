fn main() {
    println!("Twelve Days of Christmas\n");
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a patridge pear tree", "two turtle doves", "three french hens", "four calling bird", "five golden rings",
                    "six geese a-laying", "seven swans a-swiming", "eight maids a-milking", "nine ladies dancing",
                    "ten lordes a-leaping", "eleven pipers piping", "twelve drummers drumming"];

    for day in (0..12) {
        println!("on the {} day of Christmas my true love sent to me", days[day]);
        let slice = &gifts[0..day+1];
        for s in slice.into_iter().rev() {
            println!("{s}");
        }
        println!("");
    }
}

