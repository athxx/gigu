use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    User,
    Merchant,
    ServiceProvider,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::User
    }
}

impl UserRole {
    pub fn label(&self) -> &'static str {
        match self {
            Self::User => "普通用户",
            Self::Merchant => "商家",
            Self::ServiceProvider => "个人服务者",
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MeStats {
    #[serde(default)]
    pub favorites: u32,
    #[serde(default)]
    pub published: u32,
    #[serde(default)]
    pub following: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MeProfile {
    pub id: String,
    pub nickname: String,
    pub avatar_emoji: String,
    pub city: String,
    pub role: UserRole,
    #[serde(default)]
    pub available_roles: Vec<UserRole>,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub credit_score: u32,
    #[serde(default)]
    pub stats: MeStats,
}
