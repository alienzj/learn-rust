fn main() {
    let v: Vec<i32> = vec![1, 2, -3, 4, 5, 6, -7, 8, 9];
    println!("{:?}", v);
    let v2 = v.split(|n| *n < 0).collect::<Vec<_>>();
    for i in v2 {
        println!("{:?}", i);
    }

    let iter = v.iter();
    let v3 = iter.as_ref().split(|n| *n < 0).collect::<Vec<_>>();
    println!("{:?}", v3);

}
