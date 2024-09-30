# Tera Template Derive

A Rust crate for generating Tera templates with hot reloading capabilities.

## Overview

Tera is a fast, template rendering engine that allows you to separate your HTML templates from your application code. This crate provides a simple way to derive the `TeraTemplate` trait for your struct types, making it easy to integrate Tera into your existing Rust project. 

### Disclaimer

This hot-reload feature is not for compiling your Rust code. It is for generating and serving Tera templates and static files.

## Features

*   Hot reloading: automatically reloads templates when they change
*   Easy template derivation: use the `#[derive(TeraTemplate)]` macro to generate a Tera template from a simple struct type
*   Integration with Axum: provides a convenient way to serve your application with Tera templates using Axum

## Usage

### Adding the crate to your project

To add this crate to your project, run the following command:

```bash
cargo add tera-hot-reload
```

Make sure you also have the [tera](https://keats.github.io/tera/docs/) crate in your project's dependencies.

### Deriving a Tera template from a struct type

To derive a Tera template from a simple struct type, use the `#[derive(TeraTemplate)]` macro:

```rust
use tera_hot_reload::TeraTemplate;

#[derive(TeraTemplate)]
#[template(path="index.html")]
struct HelloTemplate {
    name: String,
    greeting: String,
}
```

This will generate a Tera template that looks for an `index.html` file in the `templates` directory and renders the contents of the struct.

### Creating an Axum application with hot reloading

To create an Axum application with hot reloading, use the following code:

```rust
use axum::Router;

#[tokio::main]
async fn main() {

    // Initialize the LiveReloadLayer and the reloader
    let livereload = LiveReloadLayer::new();
    let reload = livereload.reloader()

    let app = Router::new()
        .nest_service("/", get(root))
        .layer(livereload);
}
```

This will create an Axum application that serves a `root` function with hot reloading enabled.

### Reloading templates

To watch for any folder changes in your template directory and reload them automatically when they are modified, use the following code:

```rust
// after adding the LiveReloadLayer to your Axum application

let _debouncer = watch(
    move || reloader.reload(),
    Duration::from_millis(10), // if you have tailwindcss and your machine is slow, you can increase this value
    vec!["./templates"] // this is now listening for changes in the templates folder add any other folders you want to watch this can be your folder that holds your JS files or CSS or whatever you are serving in your app
)
```

### Serving the application

To serve the application, use the following code:

```rust
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app).await.unwrap();
```

This will start a TCP server on port 3000 that serves the Axum application.

## Troubleshooting

*   Make sure to add any necessary folders in the `watch()` function if used
*   Check that your template files are correctly formatted according to the Tera documentation.

## Contributing

To contribute to this project, please create a new issue or pull request on this repository. We welcome any suggestions, bug fixes, and feature requests!

## License

This project is licensed under the MIT license. See the LICENSE file for more information.

## Related Projects

*   [Axum](https://axum.rs/): A fast and modular web framework for Rust.
*   [Tera](https://github.com/keffing/tera): A fast, template rendering engine for Rust.
*   [Tower Livereload](https://github.com/tower-livereload/livereload): A live reloading layer for Tower.

Where to learn more:

*   Rust documentation: <https://doc.rust-lang.org/>
*   Axum documentation: <https://axum.rs/docs/>
*   Tera documentation: <https://keffing.github.io/tera/>