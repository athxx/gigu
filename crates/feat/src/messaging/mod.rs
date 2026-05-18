use crate::domain::location::GeoPoint;

#[derive(Clone, Debug, Default)]
pub struct ConversationPreview {
    pub id: String,
    pub title: String,
    pub unread_count: u32,
}

#[derive(Clone, Debug)]
pub enum MessagePayload {
    Text(String),
    Image(String),
    Location(GeoPoint),
}

#[derive(Clone, Debug, Default)]
pub struct MessagingState {
    pub conversations: Vec<ConversationPreview>,
    pub active_conversation_id: Option<String>,
}

pub struct MessagingModule;

impl MessagingModule {
    pub fn route() -> &'static str {
        "messaging"
    }
}
