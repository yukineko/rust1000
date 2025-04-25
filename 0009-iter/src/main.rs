fn main() {
    let mut c = vec![1,2,3,4,5,6,7,8,9,10];
    for v in c.iter_mut() {
        *v += 1;
    }
    println!("{:?}", c);
    
    let filtered = c.iter().filter(|&x| *x > 5).collect::<Vec<&i32>>();
    println!("{:?}", filtered);

    c.iter().find_map(|&x| {
        if x > 5 {
            Some(x)
        } else {
            None
        }
    }).map(|x| println!("Found: {}", x));

    let m = c.iter().max().unwrap();
    println!("Max: {}", m);

    for i in c.iter().filter_map(|&x| {if x > 5 { Some(x)} else {None}}) {
        println!("Filtered: {} ", i);
    }
    let c0 = c.iter().filter_map(|&x| {if x > 5 { Some(x)} else {None}}).collect::<Vec<i32>>();
    println!("{:?}", c0);
}