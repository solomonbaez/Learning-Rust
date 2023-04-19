use std::collections::HashMap;

fn main() {
    let mut game_1 = HashMap::new();

    game_1.insert(String::from("Blue"), 10);
    game_1.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _blue_score = game_1.get(&team_name)
        .copied()
        .unwrap_or(0);

    for (key, value) in game_1 {
        println!("{}, score: {}", key, value);
    }

    let mut game_2 = HashMap::new();
    game_2.insert(String::from("Blue"), 20);

    game_2.entry(String::from("Blue")).or_insert(10);
    game_2.entry(String::from("Yellow")).or_insert(10);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map)
}