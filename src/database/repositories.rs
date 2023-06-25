use crate::repository::user::UserRepository;

#[derive(Clone, Debug)]
pub struct Repositories {
    pub user_repository: UserRepository,
}

impl Repositories {
    pub fn new(db: mongodb::Database) -> Self {
        Self {
            user_repository: UserRepository::new(db.collection("users")),
        }
    }
}
