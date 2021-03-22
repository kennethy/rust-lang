struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("test@test.com"),
        username: String::from("kenneth"),
        active: true,
        sign_in_count: 1
    };

    println!("{}", user1.email);
}
