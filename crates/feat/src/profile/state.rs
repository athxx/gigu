use crate::profile::model::Profile;

#[derive(Clone, Debug, Default)]
pub struct ProfileState {
    pub profile: Profile,
    pub saving: bool,
}
