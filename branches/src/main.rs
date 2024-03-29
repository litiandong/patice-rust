fn main() {

    let cond = false;

    // Error:
    // let number = if cond {7 } else { "six" };
    let mut number = if cond { 7 } else { 11 };
    number %= 4;
    if number % 3 == 0  {
        println!("T1");
    }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaing = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("count = {count}");

    let a = [10, 20, 30, 40, 50];
    for i in a {
        println!("The value is: {i}");
    }

    for num in (0..5).rev() {
        println!("{num}");
    }

    println!("End");
}
