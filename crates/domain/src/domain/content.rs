use serde::{Deserialize, Serialize};

use crate::domain::user::UserRole;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublishableKind {
    Goods,
    Service,
    Post,
    Activity,
    Job,
    Housing,
    Help,
}

impl Default for PublishableKind {
    fn default() -> Self {
        Self::Post
    }
}

#[derive(Clone, Debug, Default)]
pub struct ContentVisibility {
    pub radius_meters: Option<u32>,
    pub city_level: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FeedPost {
    pub id: String,
    pub author: String,
    #[serde(default)]
    pub author_emoji: String,
    #[serde(default)]
    pub author_role: UserRole,
    pub city: String,
    pub distance_m: u32,
    pub posted_at: String,
    pub topic: String,
    pub text: String,
    #[serde(default)]
    pub likes: u32,
    #[serde(default)]
    pub comments: u32,
    #[serde(default)]
    pub spread_layer: u8,
}
