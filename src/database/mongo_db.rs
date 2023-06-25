// I want to create mongo db connection pool and use it in my routes. I have tried to use the following code:

use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{options::ClientOptions, Client, Collection};

pub struct MongoDb {
    pub client: Client,
}

impl MongoDb {
    pub async fn init() -> Self {
        dotenv().ok();
        let mongo_url = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let client_options = ClientOptions::parse(&mongo_url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        MongoDb { client }
    }

    pub fn get_client(&self) -> Client {
        self.client.clone()
    }

    pub fn get_db(&self, db_name: &str) -> mongodb::Database {
        self.client.database(db_name)
    }

    pub fn get_collection<T>(&self, db_name: &str, collection_name: &str) -> Collection<T> {
        self.client.database(db_name).collection(collection_name)
    }
}
