#[derive(Clone, Debug, Default)]
pub struct SecureStore;

impl SecureStore {
    pub fn service_name(&self) -> &'static str {
        "gigu.secure"
    }
}
