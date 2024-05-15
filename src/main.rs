use std::collections::HashMap;
use axum::{response::Html, routing::get, Router};
use axum::extract::Path;
use axum::extract::Query;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use axum::http::HeaderMap;
use axum::Extension;

struct MyCounter {
    counter: AtomicUsize,
}

struct MyConfig {
    text: String,
}

#[tokio::main]
async fn main() {
    let shared_counter = Arc::new(MyCounter {
        counter: AtomicUsize::new(0)
    });

    let shared_text = Arc::new(MyConfig {
        text: "this is my config".to_string()
    });


    let app = Router::new()
        .route("/", get(handler))
        .route("/book/:name", get(path_handler))
        .route("/book", get(query_path_handler))
        .route("/header", get(header_handler))
        .layer(Extension(shared_counter))
        .layer(Extension(shared_text));

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn header_handler(headers: HeaderMap) -> Html<String> {
    Html(format!("{headers:#?}"))
}

async fn handler(
    Extension(counter): Extension<Arc<MyCounter>>,
    Extension(config): Extension<Arc<MyConfig>>,
) -> Html<String> {
    counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("{} hello {}",
                 config.text,
                 counter.counter.load(std::sync::atomic::Ordering::Relaxed)))
}

async fn path_handler(Path(name): Path<String>) -> Html<String> {
    Html(format!("<h1>Hello, {}!</h1>", name))
}

async fn query_path_handler(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    Html(format!("{params:#?}"))
}