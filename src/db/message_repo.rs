use crate::models::message::Message;
use sqlx::SqlitePool;

// 查詢符合關鍵字條件的總筆數（用於計算總頁數）
pub async fn count_messages(pool: &SqlitePool, keyword: &str) -> i64 {
    if keyword.is_empty() {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM messages")
            .fetch_one(pool)
            .await
            .unwrap_or(0)
    } else {
        let pattern = format!("%{}%", keyword);
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM messages WHERE author LIKE ?1 OR content LIKE ?1",
        )
        .bind(&pattern)
        .fetch_one(pool)
        .await
        .unwrap_or(0)
    }
}

// 依關鍵字與分頁參數查詢留言列表
pub async fn get_messages_paginated(
    pool: &SqlitePool,
    keyword: &str,
    limit: i64,
    offset: i64,
) -> Vec<Message> {
    if keyword.is_empty() {
        sqlx::query_as::<_, Message>(
            "SELECT id, author, content, created_at FROM messages ORDER BY id DESC LIMIT ?1 OFFSET ?2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
    } else {
        let pattern = format!("%{}%", keyword);
        sqlx::query_as::<_, Message>(
            "SELECT id, author, content, created_at FROM messages WHERE author LIKE ?1 OR content LIKE ?1 ORDER BY id DESC LIMIT ?2 OFFSET ?3",
        )
        .bind(&pattern)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
    }
}

// 新增留言
pub async fn create_message(pool: &SqlitePool, author: &str, content: &str) {
    let _ = sqlx::query("INSERT INTO messages (author, content) VALUES (?1, ?2)")
        .bind(author)
        .bind(content)
        .execute(pool)
        .await;
}

// 編輯更新指定 ID 的留言
pub async fn update_message_by_id(pool: &SqlitePool, id: i64, author: &str, content: &str) {
    let _ = sqlx::query("UPDATE messages SET author = ?1, content = ?2 WHERE id = ?3")
        .bind(author)
        .bind(content)
        .bind(id)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;

    #[tokio::test]
    async fn test_message_crud_search_and_pagination() {
        let pool = init_db("sqlite::memory:").await;

        assert_eq!(count_messages(&pool, "").await, 0);

        // 1. 新增 3 筆資料測試分頁與搜尋
        create_message(&pool, "Ryowu", "Rust 後端開發筆記").await;
        create_message(&pool, "Alice", "前端 Bootstrap 樣式調整").await;
        create_message(&pool, "Bob", "Rust 非同步 SQLx 測試").await;

        assert_eq!(count_messages(&pool, "").await, 3);
        assert_eq!(count_messages(&pool, "Rust").await, 2);

        // 2. 測試分頁（每頁 2 筆，第 1 頁應為 Bob 與 Alice）
        let page1 = get_messages_paginated(&pool, "", 2, 0).await;
        assert_eq!(page1.len(), 2);
        assert_eq!(page1[0].author, "Bob");

        // 3. 測試編輯修改留言
        let target_id = page1[0].id;
        update_message_by_id(&pool, target_id, "Bob_Edited", "更新後的內容").await;

        let updated = get_messages_paginated(&pool, "Bob_Edited", 10, 0).await;
        assert_eq!(updated.len(), 1);
        assert_eq!(updated[0].content, "更新後的內容");

        // 4. 測試刪除留言
        delete_message_by_id(&pool, target_id).await;
        assert_eq!(count_messages(&pool, "").await, 2);
    }
}
