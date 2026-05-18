#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReviewTargetKind {
    User,
    Merchant,
    Listing,
    Content,
    Applet,
    WebViewUrl,
}

#[derive(Clone, Debug, Default)]
pub struct ReviewCase {
    pub id: String,
    pub target_id: String,
    pub reason: String,
}

#[derive(Clone, Debug)]
pub enum TrustSafetyAction {
    Report(ReviewCase),
    BlockUser(String),
    Moderate(ReviewTargetKind, String),
}

pub struct TrustSafetyModule;

impl TrustSafetyModule {
    pub fn route() -> &'static str {
        "trust_safety"
    }
}
