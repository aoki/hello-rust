struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    build_user(String::from("hello@example.com"), String::from("whoami"));

    let user2 = User {
        email: String::from("foo@example.com"),
        username: String::from("foo"),
        ..user1
    };
    println!("Hello, world!");

    let er_value: Result<usize, &'static str> = Err("error message");
    if let Err(e) = er_value {
        println!("err value = {}", e);
    }
}
