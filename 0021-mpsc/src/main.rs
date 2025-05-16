use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx); // txを

    thread::spawn(move || {
        let helloworld = vec![
            String::from("hello"),
            String::from("world"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for hello in helloworld {
            tx.send(hello).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });
   
    let val = rx.recv().unwrap();
    println!("Got: {}", val);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}
