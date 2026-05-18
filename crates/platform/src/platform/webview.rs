#[derive(Clone, Debug, Default)]
pub struct WebViewCapability {
    pub available: bool,
    pub js_bridge_available: bool,
}

#[derive(Clone, Debug, Default)]
pub struct WebViewBridge;

impl WebViewBridge {
    pub fn capability() -> WebViewCapability {
        WebViewCapability {
            available: cfg!(any(target_os = "ios", target_os = "android")),
            js_bridge_available: cfg!(any(target_os = "ios", target_os = "android")),
        }
    }
}
