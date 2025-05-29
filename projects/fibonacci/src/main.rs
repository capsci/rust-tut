fn main() {
    println!("Get Fibonacci number");

    println!("Enter 'n' for nth Fib number");
    let mut n = String::new();

    std::io::stdin()
        .read_line(&mut n)
        .expect("unable to read line");

    let n :i32 = n.trim().parse().expect("invalid value provided");
    println!("{n}th Fibonacci number is: {}", fib(n))
}

fn fib(mut n :i32) -> i32 {
    if n<3 {
        return 1; // first 2 fib numbers
    }
    let mut f1 = 1;
    let mut f2 = 1;
    n -= 2;
    while n != 0 {
        let t = f2;
        f2 += f1;
        f1 = t;
        n -= 1;
    }
    return f2
}
