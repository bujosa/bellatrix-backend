use std::str::FromStr;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use bson::oid::ObjectId;
use serde_json::json;

use crate::{
    constants::default_page_size::DEFAULT_PAGE_SIZE,
    database::repositories::Repositories,
    models::user::{CreateUserDto, UpdateUserDto},
    repository::user::UserRepository,
    utils::{
        http_response_builder::{
            build_bad_http_response, build_http_response, build_no_content_http_response,
            build_not_found_http_response,
        },
        mongo_error_builder::map_mongo_error,
    },
};

#[post("/register")]
async fn register(data: web::Data<Repositories>, req: web::Json<CreateUserDto>) -> impl Responder {
    let create_user_dto: CreateUserDto = req.into_inner();
    let user_repo: UserRepository = data.user_repository.clone();
    let user = user_repo.create(create_user_dto).await;

    match user {
        Ok(result) => {
            let id = result.inserted_id.as_object_id().unwrap().to_hex();
            build_http_response(json!({ "id": id }))
        }
        Err(e) => {
            let error_message = map_mongo_error(e);
            build_bad_http_response(json!({ "message": error_message }))
        }
    }
}

#[post("/login")]
async fn login(data: web::Data<Repositories>, req: web::Json<CreateUserDto>) -> impl Responder {
    let create_user_dto: CreateUserDto = req.into_inner();
    let user_repo = data.user_repository.clone();
    let verified = user_repo
        .verify_password(&create_user_dto.email, &create_user_dto.password)
        .await;

    if verified {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[get("/{id}")]
async fn get_user(data: web::Data<Repositories>, id: web::Path<String>) -> impl Responder {
    let object_id = ObjectId::from_str(&id).unwrap();
    let user_repo = data.user_repository.clone();
    let user = user_repo.get(object_id).await;

    match user {
        Some(user) => build_http_response(user),
        None => HttpResponse::NotFound().finish(),
    }
}

#[delete("/{id}")]
async fn delete_user(data: web::Data<Repositories>, id: web::Path<String>) -> impl Responder {
    let object_id = ObjectId::from_str(&id).unwrap();
    let user_repo = data.user_repository.clone();
    let deleted = user_repo.delete(object_id).await;
    if deleted {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[put("/{id}")]
async fn update_user(
    data: web::Data<Repositories>,
    id: web::Path<String>,
    req: web::Json<UpdateUserDto>,
) -> impl Responder {
    let object_id = ObjectId::from_str(&id).unwrap(); // Return early if the ID is invalid

    let user_repo = data.user_repository.clone();
    let updated = user_repo.update(object_id, req.into_inner()).await;

    match updated {
        true => build_no_content_http_response(),
        false => build_not_found_http_response(json!({
            "message": "User not found"
        })),
    }
}

#[get("/{start}/{limit}")]
async fn get_all_users(
    data: web::Data<Repositories>,
    params: web::Path<(i64, i64)>,
) -> impl Responder {
    let (start, limit) = params.into_inner();

    let user_repo = data.user_repository.clone();

    let users = user_repo.get_all(start, limit).await;

    build_http_response(users)
}

#[get("")]
async fn get_all_users_without_params(data: web::Data<Repositories>) -> impl Responder {
    let users = data.user_repository.get_all(0, DEFAULT_PAGE_SIZE).await;
    build_http_response(users)
}

pub fn user_routes() -> Scope {
    web::scope("/users")
        .service(register)
        .service(get_user)
        .service(delete_user)
        .service(update_user)
        .service(get_all_users)
        .service(get_all_users_without_params)
}
