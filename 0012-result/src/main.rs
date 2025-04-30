fn main() -> Result<(), std::io::Error> {
    let f0 = std::fs::File::open("src/main.rs")?;
    println!("File opened successfully: {:?}", f0);
        // inspect_errはエラーのResultは返さない
    
    let f1 = std::fs::File::open("src/main_.rs")
        .map_err(|e| {
            eprintln!("Error opening file: {}", e);
            e
        })?;
    println!("File opened successfully: {:?}", f1);
    
    let f2 = std::fs::File::open("src/main_.rs")
    .inspect_err(|e| {
        eprintln!("Error opening file: {}", e);
     })?;
     println!("File opened successfully: {:?}", f2);
 

    return Ok(());
}
