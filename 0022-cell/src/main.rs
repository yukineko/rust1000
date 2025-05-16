use std::cell::Cell;

#[derive(Debug)]
pub struct SomeStruct {
    some_field: Cell<i32>,
    imutalbe_field: String
}
fn main() {
    let a = SomeStruct {
        some_field: Cell::new(1),
        imutalbe_field: String::from("hello")
    };  
    a.some_field.set(3);
    println!("{:?}", a);
    println!("{}", a.some_field.get());
    println!("{}", a.imutalbe_field);

    assert_eq!(3, a.some_field.replace(100));
    assert_eq!(100, a.some_field.get());

    let v = a.some_field.into_inner();
    assert_eq!(100, v);
}
