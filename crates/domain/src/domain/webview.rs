#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UrlPolicy {
    Trusted,
    ExternalWithWarning,
    Blocked,
}

impl Default for UrlPolicy {
    fn default() -> Self {
        Self::ExternalWithWarning
    }
}

#[derive(Clone, Debug, Default)]
pub struct WebViewRequest {
    pub url: String,
    pub title: Option<String>,
    pub policy: UrlPolicy,
    pub allow_js_bridge: bool,
}
