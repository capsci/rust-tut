struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let _user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someuser123@example.com"),
        sign_in_count: 1,
    };
    // user1.email = String::from("anotheruser123@example.com"); // not allowed to modify immutable

    let mut user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someuser123@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheruser123@example.com");

    let user1 = build_user(String::from("john@example.com"), String::from("john"));

    // create user2 similar to user1 but with a different email
    let _user2 = User {
        email: String::from("john_again@example.com"),
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;

    let w1 = 30;
    let h1 = 50;
    println!("Area of rectange is {}", area(w1, h1));

    let rect = (30, 50);
    println!("Area of rectange is {}", area_with_tuple(rect));

    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30*scale),
        height: 50,
    };
    println!("Area of rectange is {}", area_of_rectangle(&rect));
    // println!("Rectangle is is {}", rect); // cannot be formatted by default formatter
                                             // trait "Debug" is not implemented
    println!("Rectangle is is {rect:?}"); // does not work until `#[derive(Debug)]` was added
    println!("Rectangle is is {rect:#?}"); // does not work until `#[derive(Debug)]` was added
    dbg!(&rect); // usable after we add `#[derive(Debug)]`

    println!("Area of rectange is {}", rect.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    let rect3 = Rectangle {
        width: 60,
        height: 40,
    };
    println!("can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3: {}", rect1.can_hold(&rect3));

    let rect = Rectangle::square(2);
    println!("Area of rectange is {}", rect.area());
}

fn build_user(email: String, username: String) -> User {
    // since parameter name match field name; we can use shorthand to initialize struct
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs; used for implementing traits
struct AlwaysEqual;

fn area(w: u32, h: u32) -> u32 {
    w * h
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width :u32,
    height :u32,
}

fn area_of_rectangle(r: &Rectangle) -> u32 {
    r.width * r.height
}

impl Rectangle {
    fn area(&self) -> u32 { // &self is shorthand for `self: &Self`
        self.width * self.height
    }
    fn width(&self) -> bool { // functions can have same name as fields; often used as getters
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self { // constructors can have any name; not necessarily "new"
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle { // you can have multiple impl blocks
    fn height(&self) -> bool {
        self.height > 0
    }

}
