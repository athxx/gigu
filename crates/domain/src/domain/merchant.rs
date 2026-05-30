use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Merchant {
    pub id: String,
    pub name: String,
    pub category: String,
    pub address: String,
    pub distance_m: u32,
    pub rating: f32,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub open_now: bool,
    pub hours: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub cover_emoji: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub lat: f64,
    #[serde(default)]
    pub lng: f64,
    #[serde(default)]
    pub intro: String,
}
