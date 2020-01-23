fn main() {
    let word = String::from("hello world!");
    let index = first_word(&word);
    println!("{}", index);
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
