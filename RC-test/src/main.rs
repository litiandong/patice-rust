use std::rc::Rc;

fn main() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    assert_eq!(t.find("taki"), Some(6), "is different");
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);


    
}
