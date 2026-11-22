use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Message {
    pub id: i64,
    pub author: String,
    pub content: String,
    pub created_at: String,
}

// 新增留言表單
#[derive(Deserialize)]
pub struct CreateMessageForm {
    pub author: String,
    pub content: String,
}

// 編輯留言表單
#[derive(Deserialize)]
pub struct UpdateMessageForm {
    pub author: String,
    pub content: String,
}

// 首頁網址查詢參數（例如 /?q=Rust&page=2）
#[derive(Deserialize, Default)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
}

// Askama 首頁模板結構（包含分頁與搜尋狀態）
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
