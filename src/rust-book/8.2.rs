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

    let hello = String::from("Hello, ");
    let world = String::from("world!");
    // let _helo_world = hello + &world;
    // // note hello has been moved here and can no longer be used

    // deref coercion
    // &String -> &str
    // &world -> &world[..]
    let _helo_world = hello + &world[..];
    // println!("{}", hello);
    println!("{}", world);


    let mut foobar = String::from("foo");
    foobar.push_str("bar");

    let mut v1 = String::from("foo");
    let v2 = "bar";
    v1.push_str(v2);
    println!("v2 is {}", v2);

    let mut v3 = String::from("lo");
    v3.push('l');
    // v3.push_str("l");
    println!("v3 is {}", v3);

    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = String::from("toe");

    // let ss = ss1 + "-" + &ss2 + "-" + &ss3;
    let _sss = format!("{}-{}-{}", ss1, ss2, ss3);
    // let h = _sss[0];

    let _len = String::from("Здравствуйте").len();
    let _xxx = "Здравствуйте";
    // let answer = &_xxx[0];
    let answer = &_xxx[0..4];
    println!("answer is {}", answer);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
   
}
