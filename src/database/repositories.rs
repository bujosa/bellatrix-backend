use crate::repository::user::UserRepository;

#[derive(Clone, Debug)]
pub struct Repositories {
    pub user_repository: UserRepository,
}

impl Repositories {
    pub async fn new(db: mongodb::Database) -> Result<Self, mongodb::error::Error> {
        let user_repository = UserRepository::new(db.collection("users")).await?;

        Ok(Self { user_repository })
    }
}
