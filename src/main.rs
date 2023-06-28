use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use bellatrix::{
    config::cors::get_cors,
    database::{mongo_db::MongoDb, repositories::Repositories},
    routes::{config::config, not_found::not_found},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mongodb = MongoDb::init().await;
    let client = mongodb.get_client();
    let db = client.database("bellatrix");
    let repositories = Repositories::new(db)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Enable the Cors
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
