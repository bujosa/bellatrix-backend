use crate::database::{mongo_db::MongoDb, repositories::Repositories};

pub async fn get_repositories() -> Result<Repositories, std::io::Error> {
    let mongodb = MongoDb::init().await;
    let client = mongodb.get_client();
    let db = client.database("bellatrix");
    Repositories::new(db).await.map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to initialize repositories: {}", err),
        )
    })
}
