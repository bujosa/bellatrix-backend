use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use bellatrix::{
    config::{cors::get_cors, data::get_repositories},
    routes::{config::config, not_found::not_found},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let repositories = get_repositories().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(repositories.clone()))
            .default_service(web::route().to(not_found))
            .configure(config)
            .wrap(get_cors())
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")
    .expect("Can not bind to port 8080")
    .run()
    .await
}
