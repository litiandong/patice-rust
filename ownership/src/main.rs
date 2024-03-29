fn main() {
    let mut s = String::from("hello");  // s 进入作用域

    // 引用
    change(&mut s);
    takes_ownership(&s);
    println!("{} world", s);

    let Str = String::from("Hello");
    let (Str, len) = get_len(Str);
    println!("Str = {Str}, len = {len}");

    let s1 = &s;
    let s2 = &s1;

    println!("{s1} {s2}");

    let s3 = &mut s;
    println!("{s3}");

    let x = 5;                      // x 进入作用域

    makes_copy(x);


    println!("x = {x}");
} 

fn get_len(word: String) -> (String, usize) {
    let len = word.len();
    (word, len)
}

fn change(word:& mut String) {
    word.push_str(" world");
}

fn takes_ownership (some_string: &String) { // some_string 进入作用域
    println!("{}", some_string);
}

fn makes_copy(some_integer: i64) { // some_integer 进入作用域
    println!("{}", some_integer);
}


