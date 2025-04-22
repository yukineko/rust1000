use reqwest;
use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()>{
    let url = "https://google.com";
    let res = reqwest::get(url).await?;
    assert!(res.status().is_success());
    println!("{}", res.status());
    let _body = res.text().await?;
    //println!("{body}");
    Ok(())
}
