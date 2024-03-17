use axum::response::Html;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    let app = Router::new()
    .merge(SwaggerUi::new("/docs")
        .url("/docs/openapi.json", ApiDoc::openapi()))
        .route("/test", get(|| async { Html("<h1>/test (work2)</h1>") }));

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, app).await.unwrap();
}
