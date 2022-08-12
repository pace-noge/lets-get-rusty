struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("user1@mail.com"),
        active: true,
        sign_in_count: 1
    };

    let user = user1.username;
    println!("{}", user);

    user1.username = String::from("wllace2");
    println!("{}", user);
    println!("{}", user1.username);

    let user2 = build_user(String::from("user2@mail.com"), String::from("user2"));
    println!("{}", user2.username);

    let user3 = User {
        email: String::from("user3@mail.com"),
        username: String::from("user3"),
        ..user2
    };

    println!("{}", user3.username);
    println!("{}", user3.active);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}