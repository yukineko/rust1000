use std::rc::Rc;

#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}
use List::{Cons, Nil};

// https://doc.rust-jp.rs/book-ja/ch15-04-rc.html
// 参照させる変数をRc::newする
// Rcでは変数はmoveを避け、参照される。参照カウントはRc::cloneごとに増える
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a = {:?}", Rc::strong_count(&a));
    let b = Rc::new(Cons(4, Rc::clone(&a)));
    println!("a = {:?}", Rc::strong_count(&a));

    {
        let c = Cons(3, Rc::clone(&b));
        println!("List(c)={:?}", c);
        println!("a = {:?}", Rc::strong_count(&a));
        println!("b = {:?}", Rc::strong_count(&b));
        let d = Cons(2, Rc::clone(&a));
        println!("List(d)={:?}", d);
        println!("a = {:?}", Rc::strong_count(&a));
        println!("b = {:?}", Rc::strong_count(&b));
    }
    println!("List(b) = {:?}", b);
    println!("a = {:?}", Rc::strong_count(&a));
    println!("b = {:?}", Rc::strong_count(&b));
}
