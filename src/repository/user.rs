use bcrypt::{hash, verify, DEFAULT_COST};
use bson::{doc, oid::ObjectId, to_document};
use futures::StreamExt;
use mongodb::{error::Error, options::FindOptions, results::InsertOneResult, Collection};
use serde::{Deserialize, Serialize};

use crate::models::user::{CreateUserDto, UpdateUserDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(collection: Collection<User>) -> Self {
        Self { collection }
    }

    pub async fn get(&self, id: ObjectId) -> Option<User> {
        let filter = doc! { "_id": id };
        self.collection.find_one(filter, None).await.unwrap()
    }

    pub async fn get_all(&self, start: i64, limit: i64) -> Vec<User> {
        let options = FindOptions::builder()
            .sort(doc! { "username": 1 })
            .skip(start.checked_mul(limit).map(|n| n as u64))
            .limit(limit as i64)
            .build();

        let mut cursor = self.collection.find(None, options).await.unwrap();

        let mut users: Vec<User> = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    users.push(document);
                }
                Err(_) => return users,
            }
        }

        users
    }

    pub async fn create(&self, create_user_dto: CreateUserDto) -> Result<InsertOneResult, Error> {
        let create_user_dto: User = User {
            id: None,
            username: create_user_dto.username,
            email: create_user_dto.email,
            profile_picture: create_user_dto.profile_picture,
            password: self.get_hashed_password(&create_user_dto.password),
        };

        let user = self
            .collection
            .insert_one(create_user_dto, None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(user)
    }

    pub async fn delete(&self, id: ObjectId) -> bool {
        let filter = doc! { "_id": id };
        let result = self.collection.delete_one(filter, None).await.unwrap();
        result.deleted_count > 0
    }

    pub async fn update(&self, id: ObjectId, update_user_dto: UpdateUserDto) -> bool {
        let filter = doc! { "_id": id };

        let update = to_document(&update_user_dto).unwrap();

        let result = self
            .collection
            .update_one(filter, update, None)
            .await
            .unwrap();
        result.modified_count > 0
    }

    pub async fn verify_password(&self, email: &str, password: &str) -> bool {
        let filter = doc! { "email": email };
        let user = self
            .collection
            .find_one(filter, None)
            .await
            .unwrap()
            .unwrap();
        verify(password, &user.password).unwrap()
    }

    fn get_hashed_password(&self, password: &str) -> String {
        hash(password, DEFAULT_COST).unwrap().to_string()
    }
}
