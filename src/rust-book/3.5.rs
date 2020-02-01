fn main() {
    let number = 12;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number2 = if condition { 5 } else { 6 };
    println!("The value of number2 is: {}", number2);

    // loop {
    //     println!("again!")
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut v = 3;

    while v != 0 {
        println!("{}!", v);

        v -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // more fast
    // more general
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for member in (1..4).rev() {
        println!("{}!", member);
    }
    println!("LIFTOFF!!!");
}
