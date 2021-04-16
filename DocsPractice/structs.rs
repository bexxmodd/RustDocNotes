struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
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
    let mut user1 = User {
        email: String::from("bmodebadze@bm.com"),
        username: String::from("bmod"),
        sign_in_count: 1,
        active: true,
    };

    let mut user2 = User {
        email: String::from("serserse@bm.com"),
        username: String::from("serserse"),
        ..user1
    };

    println!("User: {} and email: {}", user1.username, user1.email);
    user1.username = String::from("bekam");
    println!("User: {} and email: {}", user2.username, user2.email);
}