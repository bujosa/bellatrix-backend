use bson::oid::ObjectId;
use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
    pub email: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub username: String,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
    pub updated_at: Option<DateTime<Utc>>,
}
