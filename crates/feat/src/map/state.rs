use crate::map::model::MapMarker;

#[derive(Clone, Debug, Default)]
pub struct MapState {
    pub markers: Vec<MapMarker>,
    pub centered_on_user: bool,
}
