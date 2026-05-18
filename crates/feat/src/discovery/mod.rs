use crate::domain::catalog::{ListingKind, LocalListing};

#[derive(Clone, Debug, Default)]
pub struct DiscoveryState {
    pub query: String,
    pub selected_kind: Option<ListingKind>,
    pub results: Vec<LocalListing>,
}

#[derive(Clone, Debug)]
pub enum DiscoveryAction {
    Search(String),
    SelectKind(ListingKind),
    Clear,
}

pub struct DiscoveryModule;

impl DiscoveryModule {
    pub fn route() -> &'static str {
        "discovery"
    }
}
