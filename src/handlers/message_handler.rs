use axum::{
    extract::{Form, Path, Query, State},
    response::{IntoResponse, Redirect},
};
use mongodb::Database;

use crate::{
    db::message_repo,
    models::message::{
        CreateMessageForm, HtmlTemplate, IndexTemplate, SearchQuery, UpdateMessageForm,
    },
};

const PAGE_SIZE: i64 = 5;

pub async fn show_guestbook(
    State(db): State<Database>,
    Query(params): Query<SearchQuery>,
) -> impl IntoResponse {
    let keyword = params.q.unwrap_or_default().trim().to_string();
    let total_count = message_repo::count_messages(&db, &keyword).await;

    let total_pages = if total_count == 0 {
        1
    } else {
        (total_count + PAGE_SIZE - 1) / PAGE_SIZE
    };

    let current_page = params.page.unwrap_or(1).clamp(1, total_pages);
    let offset = (current_page - 1) * PAGE_SIZE;

    let messages =
        message_repo::get_messages_paginated(&db, &keyword, PAGE_SIZE, offset).await;

    HtmlTemplate(IndexTemplate {
        messages,
        query: keyword,
        current_page,
        total_pages,
        total_count,
    })
}

pub async fn add_message(
    State(db): State<Database>,
    Form(input): Form<CreateMessageForm>,
) -> impl IntoResponse {
    let author = input.author.trim();
    let content = input.content.trim();

    if !author.is_empty() && !content.is_empty() {
        message_repo::create_message(&db, author, content).await;
    }

    Redirect::to("/")
}

pub async fn edit_message(
    State(db): State<Database>,
    Path(id): Path<String>,
    Form(input): Form<UpdateMessageForm>,
) -> impl IntoResponse {
    let author = input.author.trim();
    let content = input.content.trim();

    if !author.is_empty() && !content.is_empty() {
        message_repo::update_message_by_id(&db, &id, author, content).await;
    }

    Redirect::to("/")
}

pub async fn delete_message(
    State(db): State<Database>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    message_repo::delete_message_by_id(&db, &id).await;
    Redirect::to("/")
}
