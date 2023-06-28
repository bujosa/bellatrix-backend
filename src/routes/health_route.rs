use actix_web::{get, Responder};

use crate::models::response::Response;

#[get("")]
pub async fn healthcheck() -> impl Responder {
    let response = Response {
        status: "success".to_string(),
        message: "Everything is working fine".to_string(),
    };
    actix_web::HttpResponse::Ok().json(response)
}
