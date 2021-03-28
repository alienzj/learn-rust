fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}", s, len);

    //change(&s);
    println!("{}", s);

    let mut ss = String::from("hello");
    change_mut(&mut ss);
    println!("{}", ss);

    // data races
    // let r1 = &mut ss;
    // let r2 = &mut ss;
    // println!("{}, {}", r1, r2);
    {
        let r1 = &mut ss;
        println!("{}", r1);
    }
    let r2 = &mut ss;
    println!("{}", r2);

    let sss = String::from("hello");
    let rr1 = &sss;
    let rr2 = &sss;
    println!("{}, {}", rr1, rr2);
    // let rr3 = &mut sss; // can't have a mutable reference while we have an immutable one
    // println!("{}, {}, and {}", rr1, rr2, rr3);

    // Dangling References
    // let reference_to_nothing = dangle();
    let reference_to_valid = no_dangle();
    println!("{}", reference_to_valid);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope,
  // but it does not have ownership of what it refers to, nothing happens

// As in real life, if a person owns something, you can borrow it from them.
// When you're done, you have to give it back.

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of a scope, and is dropped.
// the reference would be pointing to an invalid String

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
