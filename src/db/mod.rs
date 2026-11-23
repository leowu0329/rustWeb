pub mod message_repo;

use crate::models::message::MessageDoc;
use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, Database, IndexModel,
};

pub async fn init_db(db_url: &str, db_name: &str) -> Database {
    let client_options = ClientOptions::parse(db_url)
        .await
        .expect("無效的 MongoDB 連線字串");

    let client = Client::with_options(client_options)
        .expect("無法建立 MongoDB Client");

    let db = client.database(db_name);
    db.run_command(doc! { "ping": 1 })
        .await
        .expect("無法連線至 MongoDB Cloud，請檢查網路白名單與帳號密碼");

    // 啟動時自動建立查詢索引
    ensure_indexes(&db).await;

    db
}

// 為 messages 集合建立查詢索引以加速搜尋與排序
async fn ensure_indexes(db: &Database) {
    let collection = db.collection::<MessageDoc>("messages");

    // 1. 建立 author 欄位索引
    let author_index = IndexModel::builder()
        .keys(doc! { "author": 1 })
        .options(
            IndexOptions::builder()
                .name("idx_author".to_string())
                .build(),
        )
        .build();

    // 2. 建立 content 欄位索引
    let content_index = IndexModel::builder()
        .keys(doc! { "content": 1 })
        .options(
            IndexOptions::builder()
                .name("idx_content".to_string())
                .build(),
        )
        .build();

    // 3. 建立複合全文檢索索引
    let text_index = IndexModel::builder()
        .keys(doc! {
            "author": "text",
            "content": "text"
        })
        .options(
            IndexOptions::builder()
                .name("idx_author_content_text".to_string())
                .build(),
        )
        .build();

    let _ = collection
        .create_indexes([author_index, content_index, text_index])
        .await;
}