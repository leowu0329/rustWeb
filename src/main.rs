use rust_web::{create_app, db};
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("請在 .env 中設定 MongoDB Atlas 的 DATABASE_URL");
    let db_name = env::var("MONGODB_DB_NAME").unwrap_or_else(|_| "rust_web".to_string());
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    let database = db::init_db(&db_url, &db_name).await;
    println!("已成功連線至 MongoDB Cloud 資料庫: {}", db_name);

    let app = create_app(database);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("留言板網站已啟動！請開啟瀏覽器前往: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
