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

pub fn build_unauthorized_http_response<T>(data: T) -> HttpResponse
where
    T: Serialize,
{
    let response_data = ResponseData {
        status: "error".to_string(),
        data,
    };
    HttpResponse::Unauthorized().json(json!(response_data))
}

pub fn build_forbidden_http_response<T>(data: T) -> HttpResponse
where
    T: Serialize,
{
    let response_data = ResponseData {
        status: "error".to_string(),
        data,
    };
    HttpResponse::Forbidden().json(json!(response_data))
}

pub fn build_conflict_http_response<T>(data: T) -> HttpResponse
where
    T: Serialize,
{
    let response_data = ResponseData {
        status: "error".to_string(),
        data,
    };
    HttpResponse::Conflict().json(json!(response_data))
}

pub fn build_no_content_http_response() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::test;
    use serde_json::json;

    #[test]
    async fn test_build_http_response() {
        let response = build_http_response(json!({ "message": "Hello World" }));
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    async fn test_build_bad_http_response() {
        let response = build_bad_http_response(json!({ "message": "Hello World" }));
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn test_build_internal_server_error_http_response() {
        let response =
            build_internal_server_error_http_response(json!({ "message": "Hello World" }));
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    async fn test_build_not_found_http_response() {
        let response = build_not_found_http_response(json!({ "message": "Hello World" }));
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    async fn test_build_unauthorized_http_response() {
        let response = build_unauthorized_http_response(json!({ "message": "Hello World" }));
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    async fn test_build_forbidden_http_response() {
        let response = build_forbidden_http_response(json!({ "message": "Hello World" }));
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[test]
    async fn test_build_conflict_http_response() {
        let response = build_conflict_http_response(json!({ "message": "Hello World" }));
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[test]
    async fn test_build_no_content_http_response() {
        let response = build_no_content_http_response();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
