use std::sync::{LazyLock, RwLock};

use domain::domain::catalog::{Category, Goods, Service};
use domain::domain::content::FeedPost;
use domain::domain::merchant::Merchant;
use domain::domain::messaging::{ChatMessage, Conversation, MessageFrom};
use domain::domain::user::MeProfile;
use infra::infra::mock::{MockApi, SearchResults};

pub static MERCHANTS: LazyLock<Vec<Merchant>> = LazyLock::new(|| MockApi.list_merchants());
pub static GOODS: LazyLock<Vec<Goods>> = LazyLock::new(|| MockApi.list_goods());
pub static SERVICES: LazyLock<Vec<Service>> = LazyLock::new(|| MockApi.list_services());
pub static FEED: LazyLock<Vec<FeedPost>> = LazyLock::new(|| MockApi.list_feed());
pub static CATEGORIES: LazyLock<Vec<Category>> = LazyLock::new(|| MockApi.list_categories());
pub static ME: LazyLock<RwLock<MeProfile>> = LazyLock::new(|| RwLock::new(MockApi.me()));
pub static CONVERSATIONS: LazyLock<RwLock<Vec<Conversation>>> =
    LazyLock::new(|| RwLock::new(MockApi.list_conversations()));

pub static DISCOVER: LazyLock<RwLock<DiscoverState>> =
    LazyLock::new(|| RwLock::new(DiscoverState::default()));
pub static ACTIVE_CHAT: LazyLock<RwLock<Option<ActiveChat>>> = LazyLock::new(|| RwLock::new(None));
pub static ACTIVE_DETAIL: LazyLock<RwLock<Option<DetailTarget>>> =
    LazyLock::new(|| RwLock::new(None));

#[derive(Clone, Debug, Default)]
pub struct DiscoverState {
    pub query: String,
    pub category_id: String,
    pub results: SearchResults,
}

impl DiscoverState {
    pub fn refresh(&mut self) {
        let mut results = MockApi.search(&self.query);
        if !self.category_id.is_empty() && self.category_id != "all" {
            apply_category_filter(&mut results, &self.category_id);
        }
        self.results = results;
    }
}

fn apply_category_filter(results: &mut SearchResults, category_id: &str) {
    match category_id {
        "food" => {
            results.merchants.retain(|m| m.category.contains("早餐") || m.category.contains("咖啡"));
            results.goods.retain(|g| g.tags.iter().any(|t| matches_food(t)));
            results.services.clear();
        }
        "service" => {
            results.goods.clear();
            results.merchants.clear();
        }
        "retail" => {
            results.services.clear();
            results.merchants.retain(|m| m.category.contains("书"));
        }
        "sport" => {
            results.merchants.retain(|m| m.category.contains("瑜伽"));
            results.goods.clear();
            results.services.clear();
        }
        _ => {}
    }
}

fn matches_food(tag: &str) -> bool {
    let t = tag.to_lowercase();
    t.contains("食") || t.contains("饮") || t.contains("早餐") || t.contains("咖啡")
}

#[derive(Clone, Debug)]
pub struct ActiveChat {
    pub conversation_id: String,
    pub messages: Vec<ChatMessage>,
}

impl ActiveChat {
    pub fn open(conversation_id: &str) -> Self {
        let messages = MockApi.messages_for(conversation_id);
        if let Some(c) = CONVERSATIONS
            .write()
            .unwrap()
            .iter_mut()
            .find(|c| c.id == conversation_id)
        {
            c.unread = 0;
        }
        Self {
            conversation_id: conversation_id.to_string(),
            messages,
        }
    }

    pub fn append_me(&mut self, text: &str) {
        let text = text.trim();
        if text.is_empty() {
            return;
        }
        let msg = ChatMessage {
            from: MessageFrom::Me,
            text: text.to_string(),
            at: "刚刚".to_string(),
        };
        self.messages.push(msg.clone());
        if let Some(c) = CONVERSATIONS
            .write()
            .unwrap()
            .iter_mut()
            .find(|c| c.id == self.conversation_id)
        {
            c.last_text = msg.text.clone();
            c.last_at = msg.at.clone();
        }
    }
}

#[derive(Clone, Debug)]
pub enum DetailTarget {
    Goods(String),
    Merchant(String),
}

pub fn cycle_role() {
    let mut me = ME.write().unwrap();
    if me.available_roles.is_empty() {
        return;
    }
    let idx = me
        .available_roles
        .iter()
        .position(|r| *r == me.role)
        .unwrap_or(0);
    let next = me.available_roles[(idx + 1) % me.available_roles.len()];
    me.role = next;
}
