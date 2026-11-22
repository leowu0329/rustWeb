use axum::{
    extract::{Form, Path, Query, State},
    response::{IntoResponse, Redirect},
};
use sqlx::SqlitePool;

use crate::{
    db::message_repo,
    models::message::{
        CreateMessageForm, HtmlTemplate, IndexTemplate, SearchQuery, UpdateMessageForm,
    },
};

const PAGE_SIZE: i64 = 5; // 每頁顯示 5 則留言

// GET / ：支援關鍵字搜尋與分頁的留言板首頁
pub async fn show_guestbook(
    State(pool): State<SqlitePool>,
    Query(params): Query<SearchQuery>,
) -> impl IntoResponse {
    let keyword = params.q.unwrap_or_default().trim().to_string();
    let total_count = message_repo::count_messages(&pool, &keyword).await;

    // 計算總頁數（至少為 1 頁）
    let total_pages = if total_count == 0 {
        1
    } else {
        (total_count + PAGE_SIZE - 1) / PAGE_SIZE
    };

    // 確保頁碼落在 1 ~ total_pages 之間
    let current_page = params.page.unwrap_or(1).clamp(1, total_pages);
    let offset = (current_page - 1) * PAGE_SIZE;

    let messages =
        message_repo::get_messages_paginated(&pool, &keyword, PAGE_SIZE, offset).await;

    HtmlTemplate(IndexTemplate {
        messages,
        query: keyword,
        current_page,
        total_pages,
        total_count,
    })
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

// POST /messages/{id}/edit ：編輯修改留言
pub async fn edit_message(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Form(input): Form<UpdateMessageForm>,
) -> impl IntoResponse {
    let author = input.author.trim();
    let content = input.content.trim();

    if !author.is_empty() && !content.is_empty() {
        message_repo::update_message_by_id(&pool, id, author, content).await;
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
