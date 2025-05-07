use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)]
struct NameNumber {
    name: String,
    number: i32,
}

impl From<(String, i32)> for NameNumber {
    fn from(tuple: (String, i32)) -> Self {
        NameNumber {
            name: tuple.0,
            number: tuple.1,
        }
    }
}

fn main() {
    let tuple = ("Alice".to_string(), 42);
    let name_number: NameNumber = NameNumber::from(tuple);
    println!("{:?}", name_number);
}
