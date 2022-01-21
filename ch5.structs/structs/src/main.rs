struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
    let user1 = User {
        email: String::from("test@test.com"),
        username: String::from("kenneth"),
        active: true,
        sign_in_count: 1
    };

    println!("{}", user1.email);

    let point = Point(1, 2, 3);
    println!("{}", point.0);

    let ret = Rectangle {
        width: 30,
        height: 50
    };

    println!("{:?}", ret);
    println!("{:#?}", ret); // pretty print
    println!("area: {}", ret.area());
}
