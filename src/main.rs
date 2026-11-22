use axum::{
    extract::Path,
    response::Html,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home_page))
        .route("/hello/{name}", get(greet_user));

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("網站伺服器已啟動！請開啟瀏覽器前往: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

// 首頁：整合 Bootstrap 5 卡片排版與 Font Awesome 圖示
async fn home_page() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hant">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Rust + Bootstrap 網站</title>
            <!-- Google Fonts (Noto Sans TC) -->
            <link href="https://fonts.googleapis.com/css2?family=Noto+Sans+TC:wght@400;500;700&display=swap" rel="stylesheet">
            <!-- Bootstrap 5.3 CSS -->
            <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
            <!-- Font Awesome 6 圖示庫 -->
            <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css" rel="stylesheet">
            <style>
                body { font-family: 'Noto Sans TC', sans-serif; background-color: #f8f9fa; }
            </style>
        </head>
        <body>
            <!-- 導覽列 -->
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark shadow-sm">
                <div class="container">
                    <a class="navbar-brand fw-bold" href="/">
                        <i class="fa-brands fa-rust text-warning me-2"></i>Rust Web
                    </a>
                </div>
            </nav>

            <!-- 主要內容區 -->
            <div class="container py-5">
                <div class="row justify-content-center">
                    <div class="col-md-8 col-lg-6">
                        <div class="card shadow-sm border-0 rounded-4">
                            <div class="card-body p-4 p-md-5 text-center">
                                <div class="mb-3 text-primary">
                                    <i class="fa-solid fa-rocket fa-3x"></i>
                                </div>
                                <h1 class="h3 fw-bold mb-3">歡迎來到 Rust 網站伺服器</h1>
                                <p class="text-muted mb-4">
                                    本頁面由 <strong>Rust + Axum</strong> 後端驅動，並已成功整合
                                    <span class="badge bg-primary">Bootstrap 5</span> 與
                                    <span class="badge bg-secondary">Font Awesome</span>。
                                </p>
                                
                                <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
                                    <a href="/hello/Rust開發者" class="btn btn-primary btn-lg px-4">
                                        <i class="fa-solid fa-user-astronaut me-2"></i>測試動態路由
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Bootstrap 5.3 JS Bundle (支援下拉選單、Modal 等互動元件) -->
            <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
        </body>
        </html>
        "#,
    )
}

// 動態歡迎頁：同樣套用 Bootstrap 與 Font Awesome 樣式
async fn greet_user(Path(name): Path<String>) -> Html<String> {
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hant">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>歡迎 - {name}</title>
            <link href="https://fonts.googleapis.com/css2?family=Noto+Sans+TC:wght@400;500;700&display=swap" rel="stylesheet">
            <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
            <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css" rel="stylesheet">
            <style>
                body {{ font-family: 'Noto Sans TC', sans-serif; background-color: #f8f9fa; }}
            </style>
        </head>
        <body>
            <div class="container py-5">
                <div class="row justify-content-center">
                    <div class="col-md-6">
                        <div class="alert alert-success shadow-sm rounded-4 p-4 text-center" role="alert">
                            <i class="fa-solid fa-circle-check fa-3x mb-3"></i>
                            <h2 class="h4 fw-bold">你好，{name}！</h2>
                            <p class="mb-4">這是透過 Rust 動態路徑參數渲染的 Bootstrap 頁面。</p>
                            <a href="/" class="btn btn-outline-success">
                                <i class="fa-solid fa-arrow-left me-2"></i>返回首頁
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </body>
        </html>
        "#,
        name = name
    );

    Html(html)
}
