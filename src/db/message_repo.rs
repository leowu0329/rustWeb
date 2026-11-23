use crate::models::message::{Message, MessageDoc};
use chrono::Local;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document, Regex},
    Collection, Database,
};

const COLLECTION_NAME: &str = "messages";

fn get_collection(db: &Database) -> Collection<MessageDoc> {
    db.collection::<MessageDoc>(COLLECTION_NAME)
}

// 建立不區分大小寫的模糊搜尋過濾條件
fn build_filter(keyword: &str) -> Document {
    if keyword.is_empty() {
        doc! {}
    } else {
        // 跳脫正則特殊字元，防止使用者輸入括號或星號導致查詢報錯
        let escaped = regex_escape(keyword);
        let regex = Regex {
            pattern: escaped,
            options: "i".to_string(),
        };
        doc! {
            "$or": [
                { "author": &regex },
                { "content": &regex }
            ]
        }
    }
}

fn regex_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if "\\.+*?^$()[]{}|".contains(c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

// 查詢符合條件的總筆數
pub async fn count_messages(db: &Database, keyword: &str) -> i64 {
    let filter = build_filter(keyword);
    get_collection(db)
        .count_documents(filter)
        .await
        .unwrap_or(0) as i64
}

// 依關鍵字與分頁參數查詢留言（依 _id 降冪排序，即建立時間新到舊）
pub async fn get_messages_paginated(
    db: &Database,
    keyword: &str,
    limit: i64,
    offset: i64,
) -> Vec<Message> {
    let filter = build_filter(keyword);
    let cursor = get_collection(db)
        .find(filter)
        .sort(doc! { "_id": -1 })
        .skip(offset.max(0) as u64)
        .limit(limit)
        .await;

    match cursor {
        Ok(c) => c
            .try_collect::<Vec<MessageDoc>>()
            .await
            .unwrap_or_default()
            .into_iter()
            .map(Message::from)
            .collect(),
        Err(_) => Vec::new(),
    }
}

// 新增一筆留言
pub async fn create_message(db: &Database, author: &str, content: &str) {
    let now_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let new_doc = MessageDoc {
        id: None,
        author: author.to_string(),
        content: content.to_string(),
        created_at: now_str,
    };

    let _ = get_collection(db).insert_one(new_doc).await;
}

// 依 ObjectId 字串更新留言
pub async fn update_message_by_id(db: &Database, id: &str, author: &str, content: &str) {
    if let Ok(oid) = ObjectId::parse_str(id) {
        let _ = get_collection(db)
            .update_one(
                doc! { "_id": oid },
                doc! { "$set": { "author": author, "content": content } },
            )
            .await;
    }
}

// 依 ObjectId 字串刪除留言
pub async fn delete_message_by_id(db: &Database, id: &str) {
    if let Ok(oid) = ObjectId::parse_str(id) {
        let _ = get_collection(db).delete_one(doc! { "_id": oid }).await;
    }
}
