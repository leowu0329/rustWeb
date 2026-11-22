use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use http_body_util::BodyExt;
use rust_web::{create_app, db};
use tower::ServiceExt;

async fn read_body_string(body: Body) -> String {
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn test_guestbook_full_crud_and_search_flow() {
    let pool = db::init_db("sqlite::memory:").await;
    let app = create_app(pool);

    // 1. 新增留言
    let post_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/messages")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from("author=Ryowu&content=InitialContent"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(post_res.status(), StatusCode::SEE_OTHER);

    // 2. 編輯修改 ID=1 的留言
    let edit_res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/messages/1/edit")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from("author=Ryowu&content=UpdatedRustContent"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(edit_res.status(), StatusCode::SEE_OTHER);

    // 3. 透過關鍵字搜尋驗證修改後的內容
    let search_res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/?q=UpdatedRust")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(search_res.status(), StatusCode::OK);
    let html = read_body_string(search_res.into_body()).await;
    assert!(html.contains("UpdatedRustContent"));
    assert!(html.contains("共 1 則"));

    // 4. 搜尋不存在的關鍵字應回傳空結果提示
    let empty_search_res = app
        .oneshot(
            Request::builder()
                .uri("/?q=NonExistentKeyword")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let empty_html = read_body_string(empty_search_res.into_body()).await;
    assert!(empty_html.contains("找不到符合"));
}
