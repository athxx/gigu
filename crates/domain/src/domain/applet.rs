#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppletStatus {
    Draft,
    InReview,
    Approved,
    Rejected,
    Suspended,
}

impl Default for AppletStatus {
    fn default() -> Self {
        Self::Draft
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppletPermission {
    UserProfile,
    Location,
    Camera,
    PhotoLibrary,
    Notifications,
    Payment,
    WebView,
}

#[derive(Clone, Debug, Default)]
pub struct AppletManifest {
    pub app_id: String,
    pub name: String,
    pub version: String,
    pub entry_path: String,
    pub status: AppletStatus,
    pub required_permissions: Vec<AppletPermission>,
}
