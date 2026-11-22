use askama::Template;
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use sqlx::{sqlite::SqliteConnectOptions, FromRow, SqlitePool};
use std::str::FromStr;

#[derive(FromRow)]
struct Message {
    id: i64,
    author: String,
    content: String,
    created_at: String,
}

#[derive(Deserialize)]
struct CreateMessageForm {
    author: String,
    content: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    messages: Vec<Message>,
}

struct HtmlTemplate<T>(T);

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

#[tokio::main]
async fn main() {
    let db_options = SqliteConnectOptions::from_str("sqlite://guestbook.db")
        .unwrap()
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(db_options)
        .await
        .expect("無法連接至 SQLite 資料庫");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            author TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("建立 messages 資料表失敗");

    let app = Router::new()
        .route("/", get(show_guestbook))
        .route("/messages", post(add_message))
        .route("/messages/{id}/delete", post(delete_message))
        .with_state(pool);

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("留言板網站已啟動！請開啟瀏覽器前往: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn show_guestbook(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let messages = sqlx::query_as::<_, Message>(
        "SELECT id, author, content, created_at FROM messages ORDER BY id DESC",
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    HtmlTemplate(IndexTemplate { messages })
}

async fn add_message(
    State(pool): State<SqlitePool>,
    Form(input): Form<CreateMessageForm>,
) -> impl IntoResponse {
    let author = input.author.trim();
    let content = input.content.trim();

    if !author.is_empty() && !content.is_empty() {
        let _ = sqlx::query("INSERT INTO messages (author, content) VALUES (?1, ?2)")
            .bind(author)
            .bind(content)
            .execute(&pool)
            .await;
    }

    Redirect::to("/")
}

// POST /messages/{id}/delete ：依指定 ID 刪除資料庫中的留言
async fn delete_message(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let _ = sqlx::query("DELETE FROM messages WHERE id = ?1")
        .bind(id)
        .execute(&pool)
        .await;

    Redirect::to("/")
}
