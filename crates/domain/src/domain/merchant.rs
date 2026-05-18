#[derive(Clone, Debug, Default)]
pub struct Merchant {
    pub id: String,
    pub name: String,
    pub verified: bool,
    pub business_hours: String,
}
