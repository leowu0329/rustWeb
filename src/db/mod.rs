pub mod message_repo;

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

// 初始化 SQLite 連線池並建立資料表
pub async fn init_db(db_url: &str) -> SqlitePool {
    let db_options = SqliteConnectOptions::from_str(db_url)
        .expect("無效的資料庫連線字串")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(db_options)
        .await
        .expect("無法連接至 SQLite 資料庫");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            author TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("建立 messages 資料表失敗");

    pool
}
