use std::io;


struct User<const N: usize> {
    arr: [usize; N],
}

impl<const N: usize> User<N>{
    fn new() -> Self {
        Self {
            arr: [0; N],
        }
    }
    fn play(&mut self) {
        for i in 0..N {
            self.arr[i] = i;
        }
    }
    fn show(&self) {
        for i in 0..N {
            print!("{}", self.arr[i]);
        }
    }
}

fn main() {
    // Error!!!
    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect("Failed to read line");
    let num: usize = num.trim().parse()
        .expect("Please type a number!");
    let mut us: User<4> = User::new();
    us.play();
    us.show();
}
