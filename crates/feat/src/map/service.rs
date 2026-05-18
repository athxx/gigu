use crate::map::model::MapMarker;

pub struct MapService;

impl MapService {
    pub fn demo_marker() -> MapMarker {
        MapMarker {
            id: "demo".to_string(),
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}
