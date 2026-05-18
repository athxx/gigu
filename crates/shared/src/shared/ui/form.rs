#[derive(Clone, Debug, Default)]
pub struct FieldState {
    pub label: String,
    pub value: String,
    pub error: Option<String>,
}
