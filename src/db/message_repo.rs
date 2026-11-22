use crate::models::message::Message;
use sqlx::SqlitePool;

// 查詢所有留言
pub async fn get_all_messages(pool: &SqlitePool) -> Vec<Message> {
    sqlx::query_as::<_, Message>(
        "SELECT id, author, content, created_at FROM messages ORDER BY id DESC",
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}

// 新增一筆留言
pub async fn create_message(pool: &SqlitePool, author: &str, content: &str) {
    let _ = sqlx::query("INSERT INTO messages (author, content) VALUES (?1, ?2)")
        .bind(author)
        .bind(content)
        .execute(pool)
        .await;
}

// 依 ID 刪除留言
pub async fn delete_message_by_id(pool: &SqlitePool, id: i64) {
    let _ = sqlx::query("DELETE FROM messages WHERE id = ?1")
        .bind(id)
        .execute(pool)
        .await;
}
