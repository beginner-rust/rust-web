// handlers.rs

use axum::{
    extract::Extension,
    http::StatusCode,
    response::Html,
};
use std::sync::Arc;
use axum::response::IntoResponse;
use crate::Settings;
use tracing::{debug, info};

pub async fn static_handler(Extension(data_url_arc): Extension<Arc<String>>) -> std::result::Result<impl IntoResponse, StatusCode> {
    info!("进入static_handler");
    debug!("这是debug!!!!!");
    info!("Loaded data URL in handler: {:?}", data_url_arc);
    Ok(Html("<h1>Hello, world!</h1>".to_string()))
}
