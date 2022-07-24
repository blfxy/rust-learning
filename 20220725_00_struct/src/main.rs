struct User {
    usernmae: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// 元组结构体
struct Color(i32, i32, i32);
fn main() {
    let black = Color(0, 0, 0);
    println!("{}", black.0);

    let user1 = User {
        email: String::from("abc@123.com"),
        usernmae: String::from("Nikky"),
        active: true,
        sign_in_count: 556,
    };
    println!("email={}", user1.email);
    println!("usernmae={}", user1.usernmae);
    println!("active={}", user1.active);
    println!("sign_in_count={}", user1.sign_in_count);

    let mut user2 = User { ..user1 };
    user2.email = String::from("efg@126.com");
    user2.usernmae = String::from("xiaoming");
    println!("email={}", user2.email);
    println!("usernmae={}", user2.usernmae);
    println!("active={}", user2.active);
    println!("sign_in_count={}", user2.sign_in_count);

    let user3 = build_user(String::from("abc@123.com;"), String::from("xiaohong"));
    println!("{}", user3.usernmae);
}
fn build_user(email: String, usernmae: String) -> User {
    User {
        email,
        usernmae,
        active: true,
        sign_in_count: 0,
    }
}
