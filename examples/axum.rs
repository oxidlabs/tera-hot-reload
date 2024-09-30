use std::time::Duration;

use axum::{
    response::Html,
    routing::get,
    Router,
};

use tera_hot_reload::{TeraTemplate, watch, LiveReloadLayer};

#[tokio::main]
async fn main() {

    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let app = Router::new()
        .nest_service("/", get(root))
        .layer(livereload);
    
    let _debouncer = watch(
        move || reloader.reload(), 
        Duration::from_millis(10), 
        vec!["./templates", "./public"]
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(TeraTemplate, Serialize)]
#[template(path="index.html")] // this looks for index.html in the templates folder
struct HelloTemplate {
    name: String,
    greeting: String,
}

/* 
templates/index.html
<html lang="en">
    <head>
        <title>Example Page</title>
    </head>

    <body>
       <h1>{{ greeting }} {{ name }}!</h1>
    </body>
</html>
*/

async fn root() -> Html<String> {
    let context = HellopTemplate {
        name: "World".to_string(),
        greeting: "Hello".to_string()
    };

    Html(context.render())
}