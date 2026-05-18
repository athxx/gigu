use crate::domain::webview::{UrlPolicy, WebViewRequest};

#[derive(Clone, Debug, Default)]
pub struct WebViewState {
    pub current: Option<WebViewRequest>,
    pub can_go_back: bool,
    pub loading_progress: f32,
}

#[derive(Clone, Debug)]
pub enum WebViewAction {
    Open(WebViewRequest),
    Back,
    Refresh,
    Close,
}

pub struct WebViewModule;

impl WebViewModule {
    pub fn route() -> &'static str {
        "webview"
    }

    pub fn classify_url(url: &str) -> UrlPolicy {
        if url.starts_with("https://") {
            UrlPolicy::ExternalWithWarning
        } else {
            UrlPolicy::Blocked
        }
    }
}
