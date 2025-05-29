fn main() {
    println!("Converting from Fahrenheit to Celcius");
    println!("Please enter temperature in Fahrenheit");

    let mut f = String::new();
    std::io::stdin()
        .read_line(&mut f)
        .expect("unable to read line");

    println!("You entered: {f}");

    let f :f64 = f.trim().parse().expect("did not enter a valid value");
    println!("You entered(sanitized): {f}");

    println!("Value in Celcius is: {}", (f-32.0)/1.8)
}
