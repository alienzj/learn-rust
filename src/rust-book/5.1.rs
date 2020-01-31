struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("alienchuj@gmail.com"),
        username: String::from("alienzj"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("zhujie@genomics.cn");

    let _user2 = User {
        email: String::from("598151540@qq.com"),
        username: String::from("newbie"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let _user3 = User {
        email: String::from("lovelornpig@gmail.com"),
        username: String::from("alienzj"),
        ..user1
    };

    let _user4 = build_user("hello@world.com".to_string(), "world".to_string());

    let black = Color(0, 0, 0);
    println!{"{}", black.1};

    let origin = Point(0, 0 , 0);

    let Point(_a, _b, _c) = origin; // destructure
}

fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username,
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
