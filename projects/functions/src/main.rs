fn main() {
    let y = {
        let x = 3;
        x + 1 // notice no semicolon in the end; makes it an expression
    };
    println!("value of 'y' is: {y}");

    println!("function with return value: {}", five());
    println!("function with arg: {}", plus_one(7));
}

fn five() -> i32 { // function with return value
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}
