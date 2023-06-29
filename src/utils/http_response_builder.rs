use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct ResponseData<T> {
    status: String,
    data: T,
}

pub fn build_http_response<T>(data: T) -> HttpResponse
where
    T: Serialize,
{
    let response_data = ResponseData {
        status: "success".to_string(),
        data,
    };
    HttpResponse::Ok().json(json!(response_data))
}

pub fn build_bad_http_response<T>(data: T) -> HttpResponse
where
    T: Serialize,
{
    let response_data = ResponseData {
        status: "error".to_string(),
        data,
    };
    HttpResponse::BadRequest().json(json!(response_data))
}

pub fn build_internal_server_error_http_response<T>(data: T) -> HttpResponse
where
    T: Serialize,
{
    let response_data = ResponseData {
        status: "error".to_string(),
        data,
    };
    HttpResponse::InternalServerError().json(json!(response_data))
}

pub fn build_not_found_http_response<T>(data: T) -> HttpResponse
where
    T: Serialize,
{
    let response_data = ResponseData {
        status: "error".to_string(),
        data,
    };
    HttpResponse::NotFound().json(json!(response_data))
}
