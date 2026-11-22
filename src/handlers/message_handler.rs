use axum::{
    extract::{Form, Path, State},
    response::{IntoResponse, Redirect},
};
use sqlx::SqlitePool;

use crate::{
    db::message_repo,
    models::message::{CreateMessageForm, HtmlTemplate, IndexTemplate},
};

// GET / ：顯示留言板首頁
pub async fn show_guestbook(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let messages = message_repo::get_all_messages(&pool).await;
    HtmlTemplate(IndexTemplate { messages })
}

// POST /messages ：新增留言
pub async fn add_message(
    State(pool): State<SqlitePool>,
    Form(input): Form<CreateMessageForm>,
) -> impl IntoResponse {
    let author = input.author.trim();
    let content = input.content.trim();

    if !author.is_empty() && !content.is_empty() {
        message_repo::create_message(&pool, author, content).await;
    }

    Redirect::to("/")
}

// POST /messages/{id}/delete ：刪除留言
pub async fn delete_message(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    message_repo::delete_message_by_id(&pool, id).await;
    Redirect::to("/")
}
