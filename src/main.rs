use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use bellatrix::{
    database::mongo_db::MongoDb,
    repository::user::UserRepository,
    routes::{config::config, not_found::not_found},
};
use std::vec;

pub struct AppState {
    pub db: mongodb::Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mongodb = MongoDb::init().await;
    let client = mongodb.get_client();
    let db = client.database("bellatrix");
    let user_repository = UserRepository::new(db.collection("users"));

    // Enable the Cors
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials();

        App::new()
            .app_data(Data::new(user_repository.clone()))
            .default_service(web::route().to(not_found))
            .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")
    .expect("Can not bind to port 8080")
    .run()
    .await
}
