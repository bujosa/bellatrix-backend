use std::str::FromStr;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use bson::oid::ObjectId;

use crate::{
    models::user::{CreateUserDto, UpdateUserDto},
    repository::user::{User, UserRepository},
};

#[post("")]
async fn create_user(
    user_repo: web::Data<UserRepository>,
    req: web::Json<CreateUserDto>,
) -> impl Responder {
    let create_user_dto: CreateUserDto = req.into_inner();
    let user: Result<mongodb::results::InsertOneResult, mongodb::error::Error> =
        user_repo.create(create_user_dto).await;

    let id = user.unwrap().inserted_id.as_object_id().unwrap().to_hex();

    HttpResponse::Created().json(id)
}

#[get("/{id}")]
async fn get_user(user_repo: web::Data<UserRepository>, id: web::Path<String>) -> impl Responder {
    let object_id = ObjectId::from_str(&id).unwrap();
    let user = user_repo.get(object_id).await;
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

#[delete("/{id}")]
async fn delete_user(
    user_repo: web::Data<UserRepository>,
    id: web::Path<String>,
) -> impl Responder {
    let object_id = ObjectId::from_str(&id).unwrap();
    let deleted = user_repo.delete(object_id).await;
    if deleted {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[put("/{id}")]
async fn update_user(
    user_repo: web::Data<UserRepository>,
    id: web::Path<String>,
    req: web::Json<UpdateUserDto>,
) -> impl Responder {
    let object_id = ObjectId::from_str(&id).unwrap();

    let update_user_dto: UpdateUserDto = req.into_inner();

    let updated = user_repo.update(object_id, update_user_dto).await;

    if updated {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/{start}/{limit}")]
async fn get_all_users(
    user_repo: web::Data<UserRepository>,
    params: web::Path<(i64, i64)>,
) -> impl Responder {
    let (start, limit) = params.into_inner();
    let users = user_repo.get_all(start, limit).await;
    HttpResponse::Ok().json(users)
}

#[get("")]
async fn get_all_users_without_params(user_repo: web::Data<UserRepository>) -> impl Responder {
    let users = user_repo.get_all(0, 10).await;
    HttpResponse::Ok().json(users)
}

pub fn user_routes() -> Scope {
    web::scope("/api/users")
        .service(create_user)
        .service(get_user)
        .service(delete_user)
        .service(update_user)
        .service(get_all_users)
        .service(get_all_users_without_params)
}
