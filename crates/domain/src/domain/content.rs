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
