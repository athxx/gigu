use serde::{Deserialize, Serialize};

use crate::domain::user::UserRole;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub peer_name: String,
    #[serde(default)]
    pub peer_emoji: String,
    #[serde(default)]
    pub peer_role: UserRole,
    pub last_text: String,
    pub last_at: String,
    #[serde(default)]
    pub unread: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageFrom {
    Me,
    Peer,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub from: MessageFrom,
    pub text: String,
    pub at: String,
}
