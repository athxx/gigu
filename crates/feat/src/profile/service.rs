use crate::profile::model::Profile;

pub struct ProfileService;

impl ProfileService {
    pub fn empty_profile() -> Profile {
        Profile::default()
    }
}
