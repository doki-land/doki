#[derive(Debug, Clone)]
pub struct DokiError {
    pub kind: Box<DokiErrorKind>,
}
#[derive(Debug, Clone)]
pub enum DokiErrorKind {
    UnknownError
}

pub type Result<T> = std::result::Result<T, DokiError>;
