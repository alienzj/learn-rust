use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let _scores_2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_name);
    // println!("{}", field_value);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match scores.get(&team_name) {
        Some(score) => println!("{} score: {}", team_name, score),
        None => (),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Blue"), 50);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(60);
    scores.entry(String::from("Blue")).or_insert(60);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
