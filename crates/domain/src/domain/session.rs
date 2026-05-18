#[derive(Clone, Debug, Default)]
pub struct Session {
    pub token: Option<String>,
    pub authenticated: bool,
}
