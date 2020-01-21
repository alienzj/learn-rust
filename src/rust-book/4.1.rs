// What Is Ownership?
// The Stack and the Heap
// Ownership Rules
// Variable Scope
// The String Type
// Memory and Allocation
// Ways Variables and Data Interact: Move
// Ways Variables and Data Interact: Clone
// Stack-Only Data: Copy
// Ownership and Functions
// Return Values and Scope

fn main() {
    // Variable Scope
    let s = "world!";
    {
        let s = "hello, ";
        println!("{}", s);
    }
    println!("{}", s);

    // The String Type
    let mut s = String::from("hello");
    // Memory and Allocation
    s.push_str(", world!");
    println!("{}", s);

    // Ways Variables and Data Interact: Move
    // bind the value 5 to x,
    // then make a copy of the value in x and bind it to y
    // integers are simplevalues with a known, fixed size
    // and these two 5 values are pushed onto the stack
    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    let s1 = String::from("hello");
    // copy ptr, len, capacity on stack, not copy data on heap
    let s2 = s1;
    // println!("{}", s1); // can't run
    println!("{}", s2);

    // Ways Variables and Data Interact: Clone
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // Stack-Only Data: Copy
    // Interger type has the copy trait
    let m = 5;
    let n = m;
    println!("m = {}, n = {}", m, n);

    // So what types are Copy?
    // You can check the documentation for the given type to be sure
    // but as a general rule, any group of simple scalar values can be Copy
    // and nothing that requires allocation or is some form of resource is Copy
    // Here are some of the types that are Copy
    // - All the integer types, such as u32
    // - The Boolean type, bool, with values true and flase
    // - All the floating point types, suas as f64
    // - The character type, char
    // - Tuples, if they only contain types that are also Copy.
    // For example, (i32, i32) is Copy, but (i32, String) is not.

    // Ownership and Functions
    let ss = String::from("hello");
    takes_ownership(ss); // ss has been moved
    // println!("{}", ss);

    let xx = 5;
    makes_copy(xx);
    println!("{}", xx); // xx has the Copy  trait, so xx is still usable after assignment

    // Return Values and Scope
    let t1 = gives_ownership();
    let t2 = String::from("hello");
    let t3 = takes_and_gives_back(t2); // s2 move
    println!("{}", t1);
    // println!("{}", t2);
    println!("{}", t3);

    let sss = String::from("hello");
    let (ssss, len) = calculate_length(sss);
    println!("The length of '{}' is  {}.", ssss, len);
    // println!("{}", sss);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and drop is called

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope, nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
