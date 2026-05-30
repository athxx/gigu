use serde::{Deserialize, Serialize};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum StockState {
    #[default]
    InStock,
    FewLeft,
    OutOfStock,
    PreOrder,
}

impl StockState {
    pub fn label(&self) -> &'static str {
        match self {
            Self::InStock => "有货",
            Self::FewLeft => "少量",
            Self::OutOfStock => "无货",
            Self::PreOrder => "可预订",
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Goods {
    pub id: String,
    pub title: String,
    pub merchant_id: String,
    pub merchant_name: String,
    pub price: f32,
    #[serde(default)]
    pub original_price: f32,
    pub distance_m: u32,
    #[serde(default)]
    pub cover_emoji: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub stock_state: StockState,
    #[serde(default)]
    pub deliverable: bool,
    #[serde(default)]
    pub intro: String,
}

impl Goods {
    pub fn has_discount(&self) -> bool {
        self.original_price > self.price
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub title: String,
    pub category: String,
    pub provider_name: String,
    #[serde(default)]
    pub verified: bool,
    pub rating: f32,
    pub price_from: f32,
    pub service_mode: String,
    pub distance_m: u32,
    #[serde(default)]
    pub cover_emoji: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub intro: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub emoji: String,
}
