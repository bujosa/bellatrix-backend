use actix_web::{get, Responder};
use serde_json::json;

use crate::utils::http_response_builder::build_http_response;

#[get("")]
pub async fn healthcheck() -> impl Responder {
    build_http_response(json!({
        "message": "Server is running"
    }))
}
