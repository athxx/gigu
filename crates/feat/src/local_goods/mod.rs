use crate::domain::catalog::LocalListing;

#[derive(Clone, Debug, Default)]
pub struct GoodsState {
    pub nearby_goods: Vec<LocalListing>,
    pub selected_goods_id: Option<String>,
}

#[derive(Clone, Debug)]
pub enum GoodsAction {
    OpenDetail(String),
    ToggleFavorite(String),
    AskMerchant(String),
}

pub struct LocalGoodsModule;

impl LocalGoodsModule {
    pub fn route() -> &'static str {
        "local_goods"
    }
}
