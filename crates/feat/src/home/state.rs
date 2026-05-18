use crate::home::model::HomeCard;

#[derive(Clone, Debug, Default)]
pub struct HomeState {
    pub cards: Vec<HomeCard>,
    pub loading: bool,
}
