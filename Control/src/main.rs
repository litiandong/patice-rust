#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    //let v: Coin = Coin::Quarter(UsState::Alaska);
    let v: Coin = Coin::Dime;
    let num: u8 = value_in_cents(v);

    let x: Option<i32> = Option::Some(12);
    let n: i32 = push_value(x);
    println!("n = {n}");

    println!("num = {num}");
}

fn push_value(x: Option<i32>) -> i32 {
    match x {
        None => -1,
        Some(i) => i,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        //Coin::Penny => 1,
        Coin::Nickel => {
            println!("Lucky Nickel!");
            5
        }
        //Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        _ => other(),
    }
}

fn other() -> u8{
    println!("Other");
    10
}

