use std::thread::spawn;
use std::thread;
use std::time::Duration;
use std::vec::Vec;

fn foo() {
    //let mut v = Vec::new();
    //v.push(2);
    //v.push(3);
    
    let v = [2; 4];
    let v1 = 1;

    let ptr = &v as *const _;
    println!("vector address in function {:#x}", ptr as usize);

    let ptr1 = &v1 as *const _;
    println!("vector1 address in function {:#x}", ptr1 as usize);
    thread::spawn(move || {
        let ptr = &v as *const _;
        println!("vector address in spawn thread: {:#x}", ptr as usize);
        println!("Here's a vector: {:?}", v);
    });
}

fn doo() {
    let sdata1 = [2; 4];
    let sdata2 = [2; 4];
    let sdata3 = [2; 4];

    let ptr = &sdata1 as *const _;
    println!("vector address in function {:#x}", ptr as usize);

    let ptr = &sdata2 as *const _;
    println!("vector address in function {:#x}", ptr as usize);

    let ptr = &sdata3 as *const _;
    println!("vector address in function {:#x}", ptr as usize);
    
    let hdata = "Hello".to_string();

    let ptr = &hdata as *const _;
    println!("String address in function {:#x}", ptr as usize);

    let hdata = "Daplo".to_string();

    let ptr = &hdata as *const _;
    println!("String address in function {:#x}", ptr as usize);

}

fn main() {
    //foo();
    doo();
    println!("hi main thread");
    thread::sleep(Duration::from_millis(1));

}
