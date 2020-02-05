fn main() {
    // let mut v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let first = &v[0];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //let _does_not_exist = &v[100]; // panic
    let _does_not_exist = v.get(100);

    // v.push(9);

    println!("The first element is: {}", first);
    println!("The third element is: {}", third);

    v.push(9);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
       *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
