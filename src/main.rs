use axum::{
    extract::Path,
    response::Html,
    routing::get,
    Router,
};

// #[tokio::main] 將 main 函式轉換為非同步執行環境的進入點
#[tokio::main]
async fn main() {
    // 1. 建立路由表 (Router)，定義網址路徑對應的處理函式
    let app = Router::new()
        .route("/", get(home_page))
        .route("/hello/{name}", get(greet_user));

    // 2. 設定伺服器監聽的本機 IP 與連接埠 (Port 3000)
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    println!("網站伺服器已啟動！請開啟瀏覽器前往: http://{}", addr);

    // 3. 啟動伺服器並持續監聽請求
    axum::serve(listener, app).await.unwrap();
}

// 處理首頁請求，回傳 HTML 網頁內容
async fn home_page() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hant">
        <head>
            <meta charset="UTF-8">
            <title>我的第一個 Rust 網站</title>
            <style>
                body { font-family: sans-serif; margin: 40px; line-height: 1.6; }
                code { background: #f4f4f4; padding: 2px 6px; border-radius: 4px; }
            </style>
        </head>
        <body>
            <h1>歡迎來到 Rust 網站伺服器！</h1>
            <p>這是由 <strong>Rust + Axum</strong> 驅動的高效能網頁。</p>
            <p>試著在網址列輸入：<code>http://127.0.0.1:3000/hello/Rust開發者</code></p>
        </body>
        </html>
        "#,
    )
}

// 處理動態路徑參數，從網址中擷取 {name} 並回傳文字
async fn greet_user(Path(name): Path<String>) -> String {
    format!("你好，{}！歡迎使用 Rust 建立網站。", name)
}
