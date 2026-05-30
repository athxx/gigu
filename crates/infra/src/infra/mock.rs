use std::collections::HashMap;
use std::sync::OnceLock;

use crate::domain::catalog::{Category, Goods, Service};
use crate::domain::content::FeedPost;
use crate::domain::merchant::Merchant;
use crate::domain::messaging::{ChatMessage, Conversation};
use crate::domain::user::MeProfile;

const ME_JSON: &str = include_str!("../../../../mockdata/me.json");
const MERCHANTS_JSON: &str = include_str!("../../../../mockdata/merchants.json");
const GOODS_JSON: &str = include_str!("../../../../mockdata/goods.json");
const SERVICES_JSON: &str = include_str!("../../../../mockdata/services.json");
const FEED_JSON: &str = include_str!("../../../../mockdata/feed.json");
const CONVERSATIONS_JSON: &str = include_str!("../../../../mockdata/conversations.json");
const MESSAGES_JSON: &str = include_str!("../../../../mockdata/messages.json");
const CATEGORIES_JSON: &str = include_str!("../../../../mockdata/categories.json");

fn me_cache() -> &'static MeProfile {
    static CELL: OnceLock<MeProfile> = OnceLock::new();
    CELL.get_or_init(|| serde_json::from_str(ME_JSON).expect("parse me.json"))
}

fn merchants_cache() -> &'static [Merchant] {
    static CELL: OnceLock<Vec<Merchant>> = OnceLock::new();
    CELL.get_or_init(|| serde_json::from_str(MERCHANTS_JSON).expect("parse merchants.json"))
}

fn goods_cache() -> &'static [Goods] {
    static CELL: OnceLock<Vec<Goods>> = OnceLock::new();
    CELL.get_or_init(|| serde_json::from_str(GOODS_JSON).expect("parse goods.json"))
}

fn services_cache() -> &'static [Service] {
    static CELL: OnceLock<Vec<Service>> = OnceLock::new();
    CELL.get_or_init(|| serde_json::from_str(SERVICES_JSON).expect("parse services.json"))
}

fn feed_cache() -> &'static [FeedPost] {
    static CELL: OnceLock<Vec<FeedPost>> = OnceLock::new();
    CELL.get_or_init(|| serde_json::from_str(FEED_JSON).expect("parse feed.json"))
}

fn conversations_cache() -> &'static [Conversation] {
    static CELL: OnceLock<Vec<Conversation>> = OnceLock::new();
    CELL.get_or_init(|| serde_json::from_str(CONVERSATIONS_JSON).expect("parse conversations.json"))
}

fn messages_cache() -> &'static HashMap<String, Vec<ChatMessage>> {
    static CELL: OnceLock<HashMap<String, Vec<ChatMessage>>> = OnceLock::new();
    CELL.get_or_init(|| serde_json::from_str(MESSAGES_JSON).expect("parse messages.json"))
}

fn categories_cache() -> &'static [Category] {
    static CELL: OnceLock<Vec<Category>> = OnceLock::new();
    CELL.get_or_init(|| serde_json::from_str(CATEGORIES_JSON).expect("parse categories.json"))
}

#[derive(Clone, Debug, Default)]
pub struct MockApi;

impl MockApi {
    pub fn me(&self) -> MeProfile {
        me_cache().clone()
    }

    pub fn list_merchants(&self) -> Vec<Merchant> {
        merchants_cache().to_vec()
    }

    pub fn merchant_by_id(&self, id: &str) -> Option<Merchant> {
        merchants_cache().iter().find(|m| m.id == id).cloned()
    }

    pub fn list_goods(&self) -> Vec<Goods> {
        goods_cache().to_vec()
    }

    pub fn goods_by_id(&self, id: &str) -> Option<Goods> {
        goods_cache().iter().find(|g| g.id == id).cloned()
    }

    pub fn goods_by_merchant(&self, merchant_id: &str) -> Vec<Goods> {
        goods_cache()
            .iter()
            .filter(|g| g.merchant_id == merchant_id)
            .cloned()
            .collect()
    }

    pub fn list_services(&self) -> Vec<Service> {
        services_cache().to_vec()
    }

    pub fn service_by_id(&self, id: &str) -> Option<Service> {
        services_cache().iter().find(|s| s.id == id).cloned()
    }

    pub fn list_feed(&self) -> Vec<FeedPost> {
        feed_cache().to_vec()
    }

    pub fn list_conversations(&self) -> Vec<Conversation> {
        conversations_cache().to_vec()
    }

    pub fn conversation_by_id(&self, id: &str) -> Option<Conversation> {
        conversations_cache().iter().find(|c| c.id == id).cloned()
    }

    pub fn messages_for(&self, conversation_id: &str) -> Vec<ChatMessage> {
        messages_cache()
            .get(conversation_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn list_categories(&self) -> Vec<Category> {
        categories_cache().to_vec()
    }

    pub fn search(&self, query: &str) -> SearchResults {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return SearchResults {
                goods: self.list_goods(),
                merchants: self.list_merchants(),
                services: self.list_services(),
            };
        }
        let goods = goods_cache()
            .iter()
            .filter(|g| {
                g.title.to_lowercase().contains(&q)
                    || g.merchant_name.to_lowercase().contains(&q)
                    || g.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .cloned()
            .collect();
        let merchants = merchants_cache()
            .iter()
            .filter(|m| {
                m.name.to_lowercase().contains(&q)
                    || m.category.to_lowercase().contains(&q)
                    || m.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .cloned()
            .collect();
        let services = services_cache()
            .iter()
            .filter(|s| {
                s.title.to_lowercase().contains(&q)
                    || s.category.to_lowercase().contains(&q)
                    || s.provider_name.to_lowercase().contains(&q)
                    || s.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .cloned()
            .collect();
        SearchResults {
            goods,
            merchants,
            services,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SearchResults {
    pub goods: Vec<Goods>,
    pub merchants: Vec<Merchant>,
    pub services: Vec<Service>,
}
