use std::io::Read;
use std::fs::File;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "test.png";
    let mut f = File::open(path)?;
    let mut buf = [0; 10];
    let n = f.read(&mut buf)?;
    println!("{:?}",  &buf[..n]);
    let n = f.read(&mut buf)?;
    println!("{:?}",  &buf[..n]);

    // Read the rest of the file into a vector
    let mut buf_vec = Vec::new();
    let _ = f.read_to_end(buf_vec.as_mut())?;
    println!("{:?}",  &buf_vec[..]);
    Ok(())
}
