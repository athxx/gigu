use crate::home::model::HomeCard;

pub struct HomeService;

impl HomeService {
    pub fn default_cards() -> Vec<HomeCard> {
        vec![HomeCard {
            title: "Welcome".to_string(),
            subtitle: "This is the initial home feature scaffold.".to_string(),
        }]
    }
}
