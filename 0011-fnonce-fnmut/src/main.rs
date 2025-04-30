fn main() {
    fn consume_with_relish<T>(f: T) 
        where T:FnOnce() -> String{
            println!("{} is good", f());

    }
    let x = String::from("xyz");
    let consume_and_return_x = move || x;
    consume_with_relish(consume_and_return_x);

    fn do_twice<T>(mut f: T) 
        where T:FnMut() {
            f();
            f();
    }
    let mut y: i32 = 1;
    {
        let add_to_y = || y = y + 2;
        do_twice(add_to_y);
    }
    println!("y = {}", y);
}
