use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&b));

    *value.borrow_mut() += 10;
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
