use memcache;
fn main() {
    let client = memcache::connect("memcache://127.0.0.1:11211").unwrap();
    client.flush().unwrap();
    client.set("foo", "bar", 10).unwrap();

    let value: Option<String> = client.get("foo").unwrap();

    assert_eq!(value, Some(String::from("bar")));
    assert_eq!(value.unwrap(), "bar");

    let value_none: Option<String> = client.get("foo2").unwrap();
    assert_eq!(value_none, None);
//    assert_eq!(value_none, Some(String::from("bar")));
    println!("end");
}
