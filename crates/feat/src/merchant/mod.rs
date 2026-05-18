use crate::domain::merchant::Merchant;

#[derive(Clone, Debug, Default)]
pub struct MerchantState {
    pub selected: Option<Merchant>,
    pub followed_ids: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum MerchantAction {
    OpenStore(String),
    Follow(String),
    StartConsultation(String),
}

pub struct MerchantModule;

impl MerchantModule {
    pub fn route() -> &'static str {
        "merchant"
    }
}
