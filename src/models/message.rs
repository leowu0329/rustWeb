use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde::Deserialize;
use sqlx::FromRow;

// 資料庫資料表對應結構（欄位需加 pub 讓模板與其他模組讀取）
#[derive(FromRow)]
pub struct Message {
    pub id: i64,
    pub author: String,
    pub content: String,
    pub created_at: String,
}

// 前端表單送出結構
#[derive(Deserialize)]
pub struct CreateMessageForm {
    pub author: String,
    pub content: String,
}

// Askama 首頁模板結構
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub messages: Vec<Message>,
}

// 通用 Askama HTML 回應包裝器
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
