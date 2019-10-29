fn main()
{
    println!("Hello, world!");

    another_function(5, 6);

    let _x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);


    let a = five();
    println!("The value of a is: {}", a);

    let b = plus_one(5);
    println!("The value of b is: {}", b);

 //   let c = plus_one2(5);
 //   println!("The value of c is: {}", c);
}

fn another_function(x: i32, y: i32)
{
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32
{
    5
}

fn plus_one(x: i32) -> i32
{
    x + 1 // expression: return i32
}

//fn plus_one2(x: i32) -> i32
//{
//    x + 1; // statement: return ()
//}
