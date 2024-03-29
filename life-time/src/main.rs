
fn main() {
    let mut ware = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];
    extend(&mut ware, &head);
    extend(&mut ware, &tail);
    // 以下是错误的，违反了引用与共享原则，同时有一个只读引用和可修改引用
    /*
    for i in 0..1000 {
        extend(&mut ware, &ware);
    }
    */
}

fn extend(v: &mut Vec<f64>, m: &[f64]) {
    for elt in m {
        v.push(*elt);
    }
}

