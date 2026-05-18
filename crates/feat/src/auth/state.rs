use crate::auth::model::LoginForm;

#[derive(Clone, Debug, Default)]
pub struct AuthState {
    pub form: LoginForm,
    pub submitting: bool,
    pub error_message: Option<String>,
}
