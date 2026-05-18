#[derive(Clone, Debug)]
pub struct RouterState {
    pub current: String,
}

impl Default for RouterState {
    fn default() -> Self {
        Self {
            current: "home".to_string(),
        }
    }
}
