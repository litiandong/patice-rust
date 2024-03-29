#![allow(dead_code)]

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(c: T) -> Self {
        Self {
            calculation: c,
            value: None,
        }
    }

    fn get_value(&mut self, n: u32) -> u32 {
        match self.value {
            Some(e) => e,
            None => {
                let v = (self.calculation)(n);
                self.value = Some(v);
                v
            }
        }
    }
}


fn main() {
    let v1 = vec![1, 2, 3];
    let equar_v1 = move |x: Vec<u32>| v1 == x;

    assert!(equar_v1(vec![1, 2, 3]));
    println!("v1: {:?}", v1);


}
