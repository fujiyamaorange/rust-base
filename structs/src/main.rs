struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        // 省略できる
        email,
        username,
        is_active: true,
        sign_in_count: 1,
    }
}

// タプル構造体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username123"),
        is_active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("username567"),
        // 残りのフィールドはuser1で初期化される
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
