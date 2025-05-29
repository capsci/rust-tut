fn main() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("updated x is {x}");

    // shadowing variable is allowed; Why? doesn't it defeat the purpose of making variable
    // immutable ?
    let y = 10;
    println!("y is {y}");
    let y = 6;
    println!("updated y is {y}");
    let y = "i'm shadowed";
    println!("newly updated y is {y}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);

    // tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // "destructuring a tuple"
    println!("The value of y is: {y}");
    println!("The value of tup.1(accessed via index) is: {}", tup.1);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array is {:?}", a); // need `:?` for printing array
    let a = [3; 5];
    println!("Array is {:?}", a);

    println!("Access element at an index");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Enter index to fetch");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");
    let index:usize = index
        .trim()
        .parse()
        .expect("numeric value expected");
    let element = a[index]; // panics if index is "out of bounds"
    println!("Element at index '{index}' is: {element}")
}
