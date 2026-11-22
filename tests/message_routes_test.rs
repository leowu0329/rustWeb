use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use http_body_util::BodyExt;
use rust_web::{create_app, db};
use tower::ServiceExt; // 提供 .oneshot() 方法直接呼叫 Router

// 輔助函式：將 HTTP Response Body 轉換為 UTF-8 字串
async fn read_body_string(body: Body) -> String {
    let bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn test_guestbook_http_end_to_end_flow() {
    // 1. 初始化記憶體內獨立測試資料庫並建立 Axum 應用實例
    let pool = init_test_db().await;
    let app = create_app(pool.clone());

    // ====================================================
    // 測試步驟 1：發送 GET / 驗證初始空白狀態
    // ====================================================
    let response = app
        .clone()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let html = read_body_string(response.into_body()).await;
    assert!(html.contains("目前還沒有任何留言"), "初始頁面應顯示空狀態提示");

    // ====================================================
    // 測試步驟 2：發送 POST /messages 模擬表單送出
    // ====================================================
    // 模擬瀏覽器 application/x-www-form-urlencoded 格式（含中文字元）
    let form_body = "author=Ryowu&content=%E9%80%99%E6%98%AF%E6%95%B4%E5%90%88%E6%B8%AC%E8%A9%A6%E7%95%99%E8%A8%80";
    
    let post_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/messages")
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(Body::from(form_body))
                .unwrap(),
        )
        .await
        .unwrap();

    // 驗證伺服器回傳 303 See Other 並導向回首頁 "/"
    assert_eq!(post_response.status(), StatusCode::SEE_OTHER);
    assert_eq!(post_response.headers().get(header::LOCATION).unwrap(), "/");

    // ====================================================
    // 測試步驟 3：再次發送 GET / 驗證新留言已渲染於 HTML 中
    // ====================================================
    let response_after_post = app
        .clone()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response_after_post.status(), StatusCode::OK);
    let updated_html = read_body_string(response_after_post.into_body()).await;
    assert!(updated_html.contains("Ryowu"), "HTML 應渲染出留言者名稱");
    assert!(updated_html.contains("這是整合測試留言"), "HTML 應渲染出解碼後的中文留言內容");
    assert!(updated_html.contains("共 1 則"), "留言計數徽章應更新為 1 則");

    // ====================================================
    // 測試步驟 4：發送 POST /messages/1/delete 驗證刪除路由
    // ====================================================
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/messages/1/delete")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::SEE_OTHER);

    // 確認刪除後首頁恢復為空列表
    let final_response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let final_html = read_body_string(final_response.into_body()).await;
    assert!(final_html.contains("目前還沒有任何留言"), "刪除後頁面應恢復為空列表提示");
}

async fn init_test_db() -> sqlx::SqlitePool {
    db::init_db("sqlite::memory:").await
}
