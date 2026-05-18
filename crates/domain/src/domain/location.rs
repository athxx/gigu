#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoPoint {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LocationPrecision {
    Precise,
    Approximate,
    Hidden,
}

impl Default for LocationPrecision {
    fn default() -> Self {
        Self::Approximate
    }
}
