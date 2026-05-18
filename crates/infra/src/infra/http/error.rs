#[derive(Clone, Debug)]
pub enum HttpError {
    InvalidRequest(String),
    Transport(String),
    Decode(String),
}
