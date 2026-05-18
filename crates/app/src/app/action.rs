#[derive(Clone, Debug)]
pub enum AppAction {
    NavigateTo(&'static str),
    ShowToast(String),
    None,
}
