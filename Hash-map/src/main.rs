use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 50);
    scores.insert("Red", 100);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    let s = scores.get("Blue").copied().unwrap_or(0);
    println!("scores:{:?}", scores);
    println!("s = {:?}", s);
}

