use serde::{Deserialize, Serialize};

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
}
