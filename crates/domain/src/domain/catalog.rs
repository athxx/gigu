use crate::domain::location::GeoPoint;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListingKind {
    Goods,
    Service,
    Merchant,
    Job,
    Housing,
    Activity,
    Post,
    Applet,
}

impl Default for ListingKind {
    fn default() -> Self {
        Self::Goods
    }
}

#[derive(Clone, Debug, Default)]
pub struct LocalListing {
    pub id: String,
    pub title: String,
    pub kind: ListingKind,
    pub position: Option<GeoPoint>,
    pub distance_meters: Option<u32>,
}
