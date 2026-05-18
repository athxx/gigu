use crate::domain::user::User;

#[derive(Clone, Debug, Default)]
pub struct UserRepository;

impl UserRepository {
    pub fn current_user(&self) -> User {
        User {
            id: "demo-user".to_string(),
            name: "gigu User".to_string(),
        }
    }
}
