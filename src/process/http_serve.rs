use anyhow::Result;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
    Json
};
use axum::extract::Query;
use axum::Extension;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use axum::response::{Html, IntoResponse};
use http::HeaderMap;
use tower_http::services::ServeDir;
use tracing::{debug, info, warn};
use tower_http::trace::TraceLayer;
use crate::{Settings, static_handler};


pub async fn process_http_serve(path: PathBuf, port: u16,settings: &Settings) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);
    tracing::info!("Loaded configuration: {:?}", settings);


    let data_url = settings.database_url.clone(); // 克隆数据库 URL
    let data_url_arc = Arc::new(data_url);
    // axum router
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/book", get(static_handler))
        .layer(Extension(data_url_arc));


    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

// async fn static_handler(Extension(data_url_arc): Extension<Arc<String>>) -> std::result::Result<impl IntoResponse, StatusCode> {
//     info!("进入static_handler");
//     debug!("这是debug!!!!!");
//     info!("Loaded data URL in handler: {:?}", data_url_arc);
//     Ok(Html("<h1>Hello, world!</h1>".to_string()))
// }


