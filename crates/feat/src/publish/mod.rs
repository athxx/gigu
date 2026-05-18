use crate::domain::content::{ContentVisibility, PublishableKind};
use crate::domain::location::GeoPoint;

#[derive(Clone, Debug, Default)]
pub struct PublishDraft {
    pub kind: PublishableKind,
    pub title: String,
    pub body: String,
    pub location: Option<GeoPoint>,
    pub visibility: ContentVisibility,
}

#[derive(Clone, Debug)]
pub enum PublishAction {
    SelectKind(PublishableKind),
    SaveDraft,
    SubmitForReview,
}

pub struct PublishModule;

impl PublishModule {
    pub fn route() -> &'static str {
        "publish"
    }
}
