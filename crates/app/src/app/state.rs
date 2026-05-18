use crate::app::router::RouterState;

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub router: RouterState,
    pub session_ready: bool,
}
