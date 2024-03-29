
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


fn main() { 
    let guess : u64 = "43".parse().expect("Not a number!");
    println!("guess = {guess}");
    println!("seconds = {THREE_HOURS_IN_SECONDS}");
    
    let a = [5;5];
    for i in 0..5 {
        function(a[i]);
    }
}

fn function(x : i32) {
    println!("Value is {x}");
}

