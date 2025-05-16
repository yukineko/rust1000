use std::cell::RefCell;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let s = RefCell::new(String::from("hello"));

    let s0 = s.borrow();
    let s1 = s.borrow_mut(); // ここで実行時エラー
    Ok(())
}
