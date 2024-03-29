union Val {
    v1: usize,
    v2: u32,
    v3: u16,
    v4: u8,
}

fn main() {
    let mut val = Val{ v1: 0 };
    unsafe {
        let b = &mut val.v2;
        *b = 1 << 16;
    }
    unsafe {
        let b = &mut val.v3;
        *b = 2 << 8;
    }
    unsafe {
        let b = &mut val.v4;
        *b = 4;
    }
    unsafe {
        println!("val = {:#x}", val.v1);
    }
}


