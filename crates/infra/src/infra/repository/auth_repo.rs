#[derive(Clone, Debug, Default)]
pub struct AuthRepository;

impl AuthRepository {
    pub fn token_key(&self) -> &'static str {
        "auth_token"
    }
}
