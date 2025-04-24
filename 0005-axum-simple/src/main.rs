use axum::{
    routing::{get, post, Router},
    extract::{Json}
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}


async fn create_user(Json(payment):Json<CreateUser>) -> Json<User> {
    tracing::info!("create_user: {:?}", payment);
    let user = User {
        id: 1,
        username: payment.username,
        email: payment.email,
    };
    Json(user)
}


#[derive(serde::Deserialize, Debug)]
struct CreateUser {
    username: String,
    email: String,
}

#[derive(serde::Serialize, serde::Deserialize,Debug)]
struct User {
    id: u32,
    username: String,
    email: String,
}


