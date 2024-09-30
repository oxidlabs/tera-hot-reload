use std::time::Duration;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Serialize;
use tera_hot_reload::{watch, LiveReloadLayer, TeraTemplate};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {

    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/public", ServeDir::new("public"))
        .layer(livereload);

    let _debouncer = watch(
        move || reloader.reload(),
        Duration::from_millis(10),
        vec!["./templates", "./public"],
    );

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(TeraTemplate, Serialize)]
#[template(path = "index.html")]
struct HelloTemplate {
    name: String,
    greeting: String,
}

async fn index() -> impl IntoResponse {
    let context = HelloTemplate {
        name: "World".to_string(),
        greeting: "Hello".to_string(),
    };

    Html(context.render())
}