use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use http_body_util::BodyExt;
use rust_web::{create_app, db};
use std::env;
use tower::ServiceExt;

async fn read_body_string(body: Body) -> String {
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn test_mongodb_guestbook_flow() {
    dotenvy::dotenv().ok();
    let Ok(db_url) = env::var("DATABASE_URL") else {
        // 若 CI/CD 環境未設定雲端連線字串則自動略過
        return;
    };

    // 使用獨立的測試資料庫名稱，測試完畢後自動清理
    let test_db = db::init_db(&db_url, "rust_web_integration_test").await;
    let _ = test_db.drop().await;

    let app = create_app(test_db.clone());

    // 1. 新增留言
    let post_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/messages")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from("author=Ryowu&content=MongoCloudTest"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(post_res.status(), StatusCode::SEE_OTHER);

    // 2. 搜尋驗證內容已寫入雲端 MongoDB
    let search_res = app
        .oneshot(
            Request::builder()
                .uri("/?q=MongoCloudTest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(search_res.status(), StatusCode::OK);
    let html = read_body_string(search_res.into_body()).await;
    assert!(html.contains("MongoCloudTest"));

    // 3. 清理測試資料庫
    let _ = test_db.drop().await;
}
