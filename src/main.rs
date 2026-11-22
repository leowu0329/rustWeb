mod db;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use handlers::message_handler;

#[tokio::main]
async fn main() {
    // 1. 初始化資料庫連線池
    let pool = db::init_db("sqlite://guestbook.db").await;

    // 2. 組裝路由表
    let app = Router::new()
        .route("/", get(message_handler::show_guestbook))
        .route("/messages", post(message_handler::add_message))
        .route("/messages/{id}/delete", post(message_handler::delete_message))
        .with_state(pool);

    // 3. 啟動伺服器
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("留言板網站已啟動！請開啟瀏覽器前往: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
