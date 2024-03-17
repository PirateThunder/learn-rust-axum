use axum::response::{Html, IntoResponse};
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
        .route("/hello", get(handler_hello));

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, app).await.unwrap();
}

// #[utoipa::path(
//     get,
//     path = "/hello",
//     request_body = Todo
// )]
async fn handler_hello() -> impl IntoResponse {
    println!("->> {:12} - handler_hello", "HANDLER");

    Html("<h1>Hello World</h1>")
}
