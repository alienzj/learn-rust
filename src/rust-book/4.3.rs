fn main() {
    let word = String::from("hello world!");
    let index = first_word(&word);
    println!("{}", index);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert String to an array of bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
