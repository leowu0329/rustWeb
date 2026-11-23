pub mod db;
pub mod handlers;
pub mod models;

use axum::{
    routing::{get, post},
    Router,
};
use handlers::message_handler;
use mongodb::Database;

pub fn create_app(db: Database) -> Router {
    Router::new()
        .route("/", get(message_handler::show_guestbook))
        .route("/messages", post(message_handler::add_message))
        .route("/messages/{id}/edit", post(message_handler::edit_message))
        .route("/messages/{id}/delete", post(message_handler::delete_message))
        .with_state(db)
}
