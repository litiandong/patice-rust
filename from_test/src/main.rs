struct S {
    n: usize,
}

struct N {
    arr: [Option<&'static mut S>; 100],
}

const nol: Option<&'static mut S> = None;

impl N {
    fn new() -> Self {
        Self {
            arr: [nol; 100],
        }
    }
}

static mut KN: N = N::new();

fn main() {
    let _n = N::new();


}
