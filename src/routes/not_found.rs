use actix_web::{HttpResponse, Result};

use crate::models::response::Response;

pub async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        status: "error".to_string(),
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}
