use actix_web::{HttpResponse, Result};
use serde_json::json;

use crate::utils::http_response_builder::build_not_found_http_response;

pub async fn not_found() -> Result<HttpResponse> {
    Ok(build_not_found_http_response(json!({
        "message": "Not found"
    })))
}
