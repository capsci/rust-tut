fn main() {
    // remember: string literals are stored on a stack
    // whereas mutable string is stored on heap
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    let x = 5;
    makes_copy(x);
    println!("x {x}");// x remains in scope

    let s = String::from("hello");
    takes_ownership(s);
    // s is out of scope; accessing it throws compile time error
    // println!("s {s}");

    let s1 = gives_ownership(); // return value is moved to s1
    println!("s1 {s1}");
    let s2 = String::from("hello"); // s2 comes to scope
    println!("s2 {s2}");
    let s3 = takes_and_gives_back(s2); // s2 is moved to fn; return of fn is moved to s3
    println!("s3 {s3}");
    // println!("{s2}") // s2 is out of scope; accessing here throws compile time error

    let s1 = String::from("hello");
    let (s2,len) = calculate_length(s1); // ownership of s1 was moved
    println!("length of '{s2}' is {len}");
    // println!("{s1}"); // s1 is out of scope; accessing here throws compile time error

    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1); // ownership of s1 was NOT moved (since we passed by
                                         // reference)
    println!("length of '{s1}' is {len}");

    let s1 = String::from("hello");
    println!("{s1}");
    let mut s2 = String::from("hello");
    println!("{s2}");
    // change(&s1); // not allowed since expects a mutable reference
    // change(&mut s1); // not allowed because 's1' is not mutable
    //                  // errors as"cannot borrow as mutable"
    // change(&s2); // not allowed since accepts a mutable reference
    change(&mut s2); // not allowed since accepts a mutable reference
    println!("changed: {s2}");

    let mut s1 = String::from("hello");
    let _r1 = &mut s1;
    // let _r2 = &mut s1; // second instance of mutable reference for same variable is NOT allowed

    let r3 = &mut s2;
    println!("second mutable instance after 1st was \"moved\" {r3}");

    // another way(through scoping) to create multiple mutable references
    {
        let _r1 = &mut s1;
    }
    let _r2 = &mut s1;

    // we can have multiple immutable references; but not another mutable reference at the same time
    let mut s1 = String::from("hello");
    let _r1 = &s1; // immutable borrow
    let _r2 = &s1; // immutable borrow
    let _r3 = &mut s1; // mutable borrow; will error when/if immutable borrows are used later
    // println!("{_r1} {_r2}");
}

fn takes_ownership(some_str: String) {
    println!("some_str {some_str}");
}

fn makes_copy(some_int :i32) {
    println!("some_int {some_int}");
}

fn gives_ownership() -> String {
    let s = String::from("hello"); // 's' comes to scope
    s
} // ownership is moved

fn takes_and_gives_back(s :String) -> String { // 's' comes to scope
    s
} // ownership is moved

fn calculate_length(s :String) -> (String, usize) {
    let l = s.len();
    (s, l) // you can return a tuple
}

fn calculate_length_ref(s: &String) -> usize { // passed using reference; ownership is not moved
    s.len()
}

// not allowed since we cannot change reference values
// fn change(s: &String) {
//     s.push_str(", world!");
// }

// allowed since we are explicitly accepting a mutable reference
fn change(s: &mut String) {
    s.push_str(", world!");
}

// dangling pointers are not allowed by compiler
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // since s goes out of scope; `&s` becomes dangling pointer
// }
