use crate::domain::applet::{AppletManifest, AppletPermission};

#[derive(Clone, Debug, Default)]
pub struct AppletRuntimeState {
    pub running_app: Option<AppletManifest>,
    pub granted_permissions: Vec<AppletPermission>,
    pub last_error: Option<String>,
}

#[derive(Clone, Debug)]
pub enum AppletAction {
    Launch(String),
    Exit,
    RequestPermission(AppletPermission),
    SubmitForReview(String),
}

pub struct AppletModule;

impl AppletModule {
    pub fn route() -> &'static str {
        "applet"
    }
}
