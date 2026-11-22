use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

// 定義首頁模板結構
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

// 定義歡迎頁模板結構（欄位名稱 name 會自動對應 HTML 中的 {{ name }}）
#[derive(Template)]
#[template(path = "greet.html")]
struct GreetTemplate {
    name: String,
}

// 輔助包裝器：將 Askama 模板轉換為 Axum 的 HTTP 回應
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
    let app = Router::new()
        .route("/", get(home_page))
        .route("/hello/{name}", get(greet_user));

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("網站伺服器已啟動！請開啟瀏覽器前往: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn home_page() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate)
}

async fn greet_user(Path(name): Path<String>) -> impl IntoResponse {
    HtmlTemplate(GreetTemplate { name })
}
