use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "example.txt";
    let str = read_to_string(path)?;
    println!("{}", str);

    let path = "num.txt";
    // 改行文字もあるよね
    let num:i32 = read_to_string(path)?.trim().parse()?;
    println!("{}", num);

    Ok(())
}
