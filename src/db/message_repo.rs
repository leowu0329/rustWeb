use crate::models::message::Message;
use sqlx::SqlitePool;

// 查詢所有留言（依 ID 降冪排序，最新在最前）
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

// ==========================================
// 自動化單元測試區塊（僅在執行 cargo test 時編譯）
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;

    #[tokio::test]
    async fn test_message_crud_lifecycle() {
        // 1. 建立記憶體內獨立測試資料庫，確保與正式資料完全隔離
        let pool = init_db("sqlite::memory:").await;

        // 2. 驗證初始狀態應為空列表
        let initial_messages = get_all_messages(&pool).await;
        assert!(initial_messages.is_empty(), "新資料庫的留言列表應該為空");

        // 3. 測試新增兩筆留言
        create_message(&pool, "Ryowu", "第一則測試留言").await;
        create_message(&pool, "Rustacean", "第二則測試留言").await;

        let messages = get_all_messages(&pool).await;
        assert_eq!(messages.len(), 2, "應該要成功寫入 2 則留言");

        // 4. 驗證排序邏輯（ORDER BY id DESC，最新留言應在索引 0）
        assert_eq!(messages[0].author, "Rustacean");
        assert_eq!(messages[0].content, "第二則測試留言");
        assert_eq!(messages[1].author, "Ryowu");

        // 5. 測試刪除指定 ID 的留言
        let target_id = messages[0].id;
        delete_message_by_id(&pool, target_id).await;

        let remaining = get_all_messages(&pool).await;
        assert_eq!(remaining.len(), 1, "刪除後應該只剩 1 則留言");
        assert_eq!(remaining[0].author, "Ryowu");
    }
}
