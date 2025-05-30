fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    println!("{hello}");
    let world = &s[6..11];
    println!("{world}");

    let hello = &s[..5]; // drop first value if start index is '0'
    println!("{hello}");

    let l = s.len();
    let world = &s[6..l];
    println!("{world}");
    let world = &s[6..]; // drop last value if end index is last element of string
    println!("{world}");

    let sc = &s[..]; // copies entire string
    println!("{sc}");

    let mut s = String::from("hello world");
    let hello = first_word(&s);
    println!("{hello}");
    s.clear();
    // println!("{hello}"); // errors: "immutable borrow later used here"
                            // Rust disallow immutable reference(word) and mutable reference(s)
                            // to exists at the same time
}

fn first_word(s: &String) -> &str { // &str points to memory location by &String
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    return &s[..]
}
