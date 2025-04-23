use redis::AsyncCommands;
use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut connection = client.get_multiplexed_async_connection().await?;
    let _ : () = connection.set("my_key", "my_value").await?;
    let result:String = connection.get("my_key").await?;
    println!("{}", result);
    Ok(())
}
