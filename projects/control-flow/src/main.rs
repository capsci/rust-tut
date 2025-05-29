fn main() {
    let y = 7;

    if y < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let y = 6;

    if y%4 == 0 {
        println!("number is divisible by 4");
    } else if y%3 == 0 {
        println!("number is divisible by 3");
    } else if y%2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3 or 2");
    }

    // since `if` is an expression
    let condition = false;
    let y = if condition {5} else {6};
    // below will error since if and else have incompatible types
    // let y = if condition {5} else {"six"};
    println!("number is {y}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // WHAT?? break is an expression?
            // both `break 111;` and `break 111` produce same results ?
            break 111
        }
    };
    println!("value of result is: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count {count}");
        let mut remaining = 10;
        loop {
            println!("remaining {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("count is: {count}");

    let mut y = 3;
    while y != 0 {
        println!("{y}");
        y -= 1;
    }
    println!("LIFT OFF!!");

    let a = [1,2,3,4,5];
    let mut i = 0;
    while i < 5 {
        println!("element at index '{i}' is: {}", a[i]);
        i += 1;
    }

    for element in a {
        println!("element is {element}");
    }

    // rev is used to reverse a range
    for number in (1..4).rev() {
        // interestingly, (1..4) produces [3,2,1] :(
        println!("{number}");
    }
    println!("LIFT OFF!!!");
}
