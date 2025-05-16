use std::thread;
use std::sync::{Arc, Mutex};
#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(Arc<Mutex<i32>>, Arc<List>),
    Nil,
}
use List::{Cons, Nil};
fn main() {
    let value = Arc::new(Mutex::new(5));
    let a = Arc::new(Cons(Arc::clone(&value), Arc::new(Nil)));
    let b = Arc::new(Cons(Arc::new(Mutex::new(3)), Arc::clone(&a)));
    let c = Cons(Arc::new(Mutex::new(4)), Arc::clone(&b));
    let mut handlers = vec![];
    for _ in 1..10 {
        let value = Arc::clone(&value);
        let handle = thread::spawn(move || {
            let mut a = value.lock().unwrap();
            *a += 1;            
        });
        handlers.push(handle);
    }
    for h in handlers {
        h.join().unwrap();
    }   
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

}

