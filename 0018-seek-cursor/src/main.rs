use std::fs::File;
use std::io::{Read, Seek};
use std::io::SeekFrom;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("example.txt")?;
    // 最後まで読む
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // 100 bytes戻る
    f.seek(SeekFrom::Current(-100))?;
    let mut buffer2 = [0; 100];
    f.read(&mut buffer2)?;  
    let str = String::from_utf8(buffer2.to_vec())?;
    println!("Buffer: {:?}", buffer2);
    println!("String: {:?}", str);
    Ok(())
}
