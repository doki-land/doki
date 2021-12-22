#[derive(Debug, Clone)]
pub struct GitError {
    pub kind: Box<GitErrorKind>,
}
#[derive(Debug, Clone)]
pub enum GitErrorKind {
    UnknownError,
}

pub type Result<T> = std::result::Result<T, GitError>;
