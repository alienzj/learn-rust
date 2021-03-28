use bio::io::{fasta, fastq};

fn main() {
    println!("Hello, world!");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);

    println!("The value of y is: {}", y);

    println!("The value of z is: {}", z);

    let g: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = g.0;
    let six_point_four = g.1;
    let one = g.2;
}
