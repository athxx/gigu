use crate::auth::model::LoginForm;

pub struct AuthService;

impl AuthService {
    pub fn validate(form: &LoginForm) -> bool {
        !form.email.trim().is_empty() && !form.password.is_empty()
    }
}
