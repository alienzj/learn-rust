fn main() {
    let mut word = String::from("hello world!");
    let index = first_word(&word);
    word.clear();
    println!("{}", index);

    // String Slices
    let s = String::from("hello world");
    let h = &s[0..5];
    let w = &s[6..11];
    // s.clear();
    // Diagam
    
    //       s
    // name     value             index value
    // ptr      -------->         0     h
    // len      11                1     e
    // capacity 11                2     l
    //                            3     l
    //       w                    4     o
    // name     value             5
    // ptr      -------->         6     w
    // len      5                 7     o
    //                            8     r
    //                            9     l
    //                            10    d

    println!("{}, {}!", h, w);

    let my_string = String::from("hello world");

    let _word = first_world_v2(&my_string[..]);

    // let word = first_world_v2(my_string);

    let my_string_literal = "hello world";

    let _word = first_world_v2(&my_string_literal[..]);

    let _word = first_world_v2(my_string_literal);

    // Other Slices
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];

    // The concepts of ownership, borrowing, and slices
    // ensure memory safety in Rust programs at compile time.
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert String to an array of bytes

    // create iterator over the array of bytes using the iter method
    // (index of the element, reference to the element)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_world_v2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
