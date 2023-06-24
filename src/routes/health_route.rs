use actix_web::{get, Responder};

use crate::models::response::Response;

#[get("/api/health")]
pub async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    actix_web::HttpResponse::Ok().json(response)
}
