use std::thread::spawn;
use std::thread;
use std::time::Duration;

fn foo() {
    let v = vec![1, 2, 3];
    thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
}

fn main() {
    foo();
    println!("hi main thread");
    thread::sleep(Duration::from_millis(1));

}
