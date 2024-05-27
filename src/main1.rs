use std::collections::HashMap;
use axum::{response::Html, routing::get, Router, Json};
use axum::extract::{Path, State};
use axum::extract::Query;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use axum::http::{HeaderMap, StatusCode};
use axum::Extension;
use axum::response::IntoResponse;
use tower_http::services::ServeDir;
use log::info;
use tower_http::trace::TraceLayer;
struct MyCounter {
    counter: AtomicUsize,
}
struct Counter {
    count: AtomicUsize,
}
struct MyConfig {
    text: String,
}

struct MyState(i32);
fn service_one()->Router {
    let state: Arc<MyState> = Arc::new(MyState(5));
    Router::new().route("/", get(sv1_handler)).with_state(state)
}

async fn sv1_handler(
    Extension(counter): Extension<Arc<MyCounter>>,
    State(state): State<Arc<MyState>>,
) -> Html<String> {
    counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("service{}:{}", counter.counter.load(std::sync::atomic::Ordering::Relaxed), state.0))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("starting server");
    let shared_counter = Arc::new(MyCounter {
        counter: AtomicUsize::new(0)
    });
    let counter = Arc::new(Counter {
        count: AtomicUsize::new(0)
    });


    let shared_text = Arc::new(MyConfig {
        text: "this is my config".to_string()
    });

    let app = Router::new()
        // .nest("/1", service_one())
        //  .route("/", get(handler))
        //  .route("/book/:name", get(path_handler))
         .route("/book", get(query_path_handler))
        .layer(TraceLayer::new_for_http());

    // .route("/header", get(header_handler))
    //     .route("/inc", get(increment))
    //     .with_state(counter)
    //     .route("/static",get(static_handler))
    //     .fallback_service(ServeDir::new("web"));

        // .layer(Extension(shared_counter))
        // .layer(Extension(shared_text));

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn static_handler() -> Result<impl IntoResponse, StatusCode> {
    Ok( Html("<h1>Hello, world!</h1>".to_string()))

}
async fn header_handler(headers: HeaderMap) -> Html<String> {
    Html(format!("{headers:#?}"))
}

async fn handler1(
    Extension(counter): Extension<Arc<MyCounter>>,
    Extension(config): Extension<Arc<MyConfig>>,
) -> Html<String> {
    counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("{} hello {}",
                 config.text,
                 counter.counter.load(std::sync::atomic::Ordering::Relaxed)))
}
async fn handler() -> Result<impl IntoResponse, (StatusCode, String)> {
   let start =std::time::SystemTime::now();
    let second_warpped = start
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_|(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get current time".to_string()))?
        .as_secs() % 3;
    let divided = 100u64.checked_div(second_warpped)
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Failed to divide".to_string()))?;
    Ok(Json(divided))
}
async fn path_handler(Path(name): Path<String>) -> Html<String> {
    Html(format!("<h1>Hello, {}!</h1>", name))
}

async fn query_path_handler(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    tracing::info!("hello world");

    tracing::debug!("hello world");
    Html(format!("{params:#?}"))
}

async fn increment (State(counter): State<Arc<Counter>>) -> Json<usize> {
    println!("/inc service called");
    let current = counter.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Json(current)
}

async fn call_inc()-> Html<String> {
    println!("/inc called");
    let count = reqwest::get("http://localhost:3000/inc").await.unwrap().text().await.unwrap();

    Html(format!("<h1>/inc called:{count}</h1>"))
}