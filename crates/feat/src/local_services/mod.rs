use crate::domain::catalog::LocalListing;

#[derive(Clone, Debug, Default)]
pub struct LocalServicesState {
    pub services: Vec<LocalListing>,
    pub selected_service_id: Option<String>,
}

#[derive(Clone, Debug)]
pub enum LocalServicesAction {
    OpenService(String),
    RequestBooking(String),
    ContactProvider(String),
}

pub struct LocalServicesModule;

impl LocalServicesModule {
    pub fn route() -> &'static str {
        "local_services"
    }
}
