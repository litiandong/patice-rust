fn main() {
    let mut s = String::from("Hello world");
    let index =  first_word(&s);
    println!("index = {index}");
    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    println!("s = {s}");

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

