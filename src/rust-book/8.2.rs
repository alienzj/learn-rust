fn main() {
    let mut _s = String::new();

    let data = "initial contents";

    // String::from and to_string do the same thing
    let _s2 = data.to_string();
    let _s3 = "initial contents".to_string();
    let _s4 = String::from("initial contents");

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("_Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    let _hello_world = String::from("hello") + &String::from(" world");

    let mut foobar = String::from("foo");
    foobar.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}
