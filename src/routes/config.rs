use super::user::user_routes;
use crate::routes::health_route;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/health").service(health_route::healthcheck));
    cfg.service(user_routes());
}
