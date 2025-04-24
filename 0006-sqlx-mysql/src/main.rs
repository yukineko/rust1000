use sqlx::{Executor};
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://sample:password@localhost:3306/sample")
        .await
        .unwrap();
    pool.execute("DROP TABLE IF  EXISTS users")
        .await
        .unwrap();
    pool.execute("CREATE TABLE IF NOT EXISTS users (id integer PRIMARY KEY AUTO_INCREMENT, username VARCHAR(255), email VARCHAR(255))")
        .await
        .unwrap();
    pool.execute("INSERT INTO users (username, email) VALUES ('testuser', 'aaa@example.com')")
        .await
        .unwrap();
    let row: (i32, String, String) = sqlx::query_as("SELECT id, username, email FROM users WHERE id = ?")
        .bind(1)
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("User: id={}, username={}, email={}", row.0, row.1, row.2);
}

