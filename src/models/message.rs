use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// 1. 對應 MongoDB Collection 儲存的文件結構
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDoc {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: String,
    pub content: String,
    pub created_at: String,
}

// 2. 傳遞給 Askama 前端模板的視圖模型（保持欄位名稱與原本完全一致）
pub struct Message {
    pub id: String,
    pub author: String,
    pub content: String,
    pub created_at: String,
}

impl From<MessageDoc> for Message {
    fn from(doc: MessageDoc) -> Self {
        Self {
            id: doc.id.map(|oid| oid.to_hex()).unwrap_or_default(),
            author: doc.author,
            content: doc.content,
            created_at: doc.created_at,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateMessageForm {
    pub author: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateMessageForm {
    pub author: String,
    pub content: String,
}

#[derive(Deserialize, Default)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub messages: Vec<Message>,
    pub query: String,
    pub current_page: i64,
    pub total_pages: i64,
    pub total_count: i64,
}

pub struct HtmlTemplate<T>(pub T);

impl<T: Template> IntoResponse for HtmlTemplate<T> {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("模板渲染失敗: {}", err),
            )
                .into_response(),
        }
    }
}
