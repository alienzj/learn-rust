fn main() {
    // scalar
    // integer
    //let guess = "43".parse().expect("Not a number!");
    let guess: u32 = "43".parse().expect("Not a number!");
    println!("{}", guess);

    // Length  Signed  Unsigned
    // 8-bit   i8      u8
    // 16-bit  i16     u16
    // 32-bit  i32     u32
    // 64-bit  i64     u64
    // 128-bit i128    u128
    // arch    isize   usize
    // Each signed variant can store numbers from -2^(n-1) to 2^(n-1) - 1
    // Unsigned variants can store numbers from 0 to 2^n - 1

    // Number literals    Example
    // Decimal            98_222
    // Hex                0xff
    // Octal              0o77
    // Binary             0b1111_0000
    // Byte (u8 only)     b'A'

    // floating=point
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    // numeric operations
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    // remainder
    let _remainder = 43 % 5;

    // boolean
    let _t = true;
    let _f: bool = false;

    // character
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x1, _x2, x3) = tup;
    println!("The value of y is: {}", x3);

    let n: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = n.0;
    let _six_point_four = n.1;
    let _one = n.2;

    // array
    let _m = [1, 2, 3, 4, 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a2 = [3; 5];
    let _first = _a[0];
    let _second = _a[1];
    // let _element = _a[10];
}
