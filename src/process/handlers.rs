// handlers.rs

use axum::{extract::Extension, http::StatusCode, Json, response::Html};
use std::sync::Arc;
use axum::extract::Query;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use crate::Settings;
use tracing::{debug, error, info};
use serde::{Deserialize, Serialize, Serializer};
use chrono::NaiveDateTime;
use anyhow::Context;
use serde::ser::SerializeStruct;


#[derive(Debug, Deserialize,sqlx::FromRow)]
pub struct DefaultApp {
    defaultAppId: i32,
    jobCard: Option<String>,
    nginxAppName: Option<String>,
    isDelete: Option<String>,
    inputDate: Option<NaiveDateTime>,
}

impl Serialize for DefaultApp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("DefaultApp", 5)?;

        state.serialize_field("defaultAppId", &self.defaultAppId)?;
        state.serialize_field("jobCard", &self.jobCard)?;
        state.serialize_field("nginxAppName", &self.nginxAppName)?;
        state.serialize_field("isDelete", &self.isDelete)?;

        // Serialize inputDate if it contains a value, otherwise serialize as null
        if let Some(input_date) = &self.inputDate {
            let formatted_date = input_date.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
            state.serialize_field("inputDate", &formatted_date)?;
        } else {
            state.serialize_field("inputDate", &Option::<String>::None)?;
        }

        state.end()
    }
}

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}
pub struct AppState {
    pub(crate) db_pool: sqlx::MySqlPool,
}



pub async fn static_handler(Extension(data_url_arc): Extension<Arc<String>>) -> std::result::Result<impl IntoResponse, StatusCode> {
    info!("进入static_handler");
    debug!("这是debug!!!!!");
    info!("Loaded data URL in handler: {:?}", data_url_arc);
    Ok(Html("<h1>Hello, world!</h1>".to_string()))
}


pub async fn get_apps(
    Query(pagination): Query<Pagination>,
    Extension(db_state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<DefaultApp>>, (StatusCode, &'static str)> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let apps = sqlx::query_as!(
        DefaultApp,
        r#"
        SELECT defaultAppId, jobCard, nginxAppName, isDelete, inputDate
        FROM DefaultApp
        LIMIT ? OFFSET ?
        "#,
        per_page as i64,
        offset as i64
    )
        .fetch_all(&db_state.db_pool)
        .await
        .context("Database error")
        .map_err(|e| {
            error!("Failed to fetch apps: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
        })?;

    // 将 JSON 值序列化为字符串，用于日志记录
    let apps_json_str = serde_json::to_string(&apps)
        .map_err(|e| {
            error!("Failed to serialize apps to JSON: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Serialization error")
        })?;

    // 打印 info 日志
    info!("Fetched apps from database: {}", apps_json_str);
    Ok(Json(apps))
}