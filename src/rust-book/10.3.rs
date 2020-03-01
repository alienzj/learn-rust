use std::fmt::Display;

fn main() {
    let r;
    //{
    //   let x = 5;
    //   r = &x;
    //}

    let x = 5;
    r = &x;

    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    main_test();

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lonagest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main_test() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

//fn main_test2() {
//    let string1 = String::from("long string is long");
//    let result;
//    {
//        let string2 = String::from("xyz");
//        result = longest(string1.as_str(), string2.as_str());
//    }
//    println!("The longest string is {}", result);
//}

//fn longest<'a>(x: &'a str, y: &str) -> &'a str {}
//fn longest<'a>(x: &str, y: &str) -> &'a str {}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
