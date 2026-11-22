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

// 首頁：加入 Bootstrap 表單輸入框與自動跳轉腳本
async fn home_page() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hant">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Rust + Bootstrap 網站</title>
            <link href="https://fonts.googleapis.com/css2?family=Noto+Sans+TC:wght@400;500;700&display=swap" rel="stylesheet">
            <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">
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
                                <h1 class="h3 fw-bold mb-2">歡迎來到 Rust 網站伺服器</h1>
                                <p class="text-muted mb-4">
                                    請在下方輸入你的名字，體驗動態路由跳轉效果
                                </p>

                                <!-- Bootstrap 表單輸入框 -->
                                <form id="greetForm" class="mx-auto" style="max-width: 400px;">
                                    <div class="input-group input-group-lg shadow-sm">
                                        <span class="input-group-text bg-white text-muted">
                                            <i class="fa-solid fa-user"></i>
                                        </span>
                                        <input 
                                            type="text" 
                                            id="userNameInput" 
                                            class="form-control" 
                                            placeholder="請輸入您的名字..." 
                                            required 
                                            autofocus
                                        >
                                        <button type="submit" class="btn btn-primary px-4">
                                            送出 <i class="fa-solid fa-arrow-right ms-1"></i>
                                        </button>
                                    </div>
                                </form>

                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <!-- 處理表單送出並跳轉至 /hello/{name} -->
            <script>
                document.getElementById('greetForm').addEventListener('submit', function(event) {
                    // 阻止表單預設的重新整理行為
                    event.preventDefault();
                    
                    const inputVal = document.getElementById('userNameInput').value.trim();
                    if (inputVal) {
                        // 使用 encodeURIComponent 確保中文與特殊符號能安全放入網址路徑
                        window.location.href = '/hello/' + encodeURIComponent(inputVal);
                    }
                });
            </script>
            <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js"></script>
        </body>
        </html>
        "#,
    )
}

// 動態歡迎頁：渲染個人化歡迎畫面
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
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark shadow-sm">
                <div class="container">
                    <a class="navbar-brand fw-bold" href="/">
                        <i class="fa-brands fa-rust text-warning me-2"></i>Rust Web
                    </a>
                </div>
            </nav>

            <div class="container py-5">
                <div class="row justify-content-center">
                    <div class="col-md-6">
                        <div class="card border-0 shadow-sm rounded-4 p-4 p-md-5 text-center">
                            <div class="text-success mb-3">
                                <i class="fa-solid fa-circle-check fa-4x"></i>
                            </div>
                            <h2 class="h3 fw-bold mb-3">你好，{name}！</h2>
                            <p class="text-muted mb-4">歡迎光臨，這是專屬於你的 Rust 動態歡迎頁面。</p>
                            <div>
                                <a href="/" class="btn btn-outline-primary px-4">
                                    <i class="fa-solid fa-arrow-left me-2"></i>重新輸入名字
                                </a>
                            </div>
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
