enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        } // When a Coin::Quarter matches,
          // the state variable will bind to the value of that quarter's state.
    }
}

fn plus_one(one: Option<i32>) -> Option<i32> {
    match one {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let _five = Some(5);
    let _six = plus_one(_five);
    let _none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
