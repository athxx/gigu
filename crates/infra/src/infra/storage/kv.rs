#[derive(Clone, Debug, Default)]
pub struct KvStore;

impl KvStore {
    pub fn namespace(&self) -> &'static str {
        "app"
    }
}
