use std::fs::File;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _fd = File::open("./test.txt")?;
    let _env = std::env::var("RUST_BACKTRACE")?;
    Ok(())
}
