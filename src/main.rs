mod db;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use handlers::message_handler;
use std::env;

#[tokio::main]
async fn main() {
    // 1. 載入 .env 檔案中的環境變數（若檔案不存在則忽略，改讀系統環境變數）
    dotenvy::dotenv().ok();

    // 2. 讀取設定參數並提供預設值
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://guestbook.db".to_string());
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    // 3. 初始化資料庫連線池
    let pool = db::init_db(&db_url).await;

    // 4. 組裝路由表
    let app = Router::new()
        .route("/", get(message_handler::show_guestbook))
        .route("/messages", post(message_handler::add_message))
        .route("/messages/{id}/delete", post(message_handler::delete_message))
        .with_state(pool);

    // 5. 啟動伺服器
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("留言板網站已啟動！請開啟瀏覽器前往: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
