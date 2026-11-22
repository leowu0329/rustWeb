use rust_web::{create_app, db};
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://guestbook.db".to_string());
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    let pool = db::init_db(&db_url).await;
    let app = create_app(pool);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("留言板網站已啟動！請開啟瀏覽器前往: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
