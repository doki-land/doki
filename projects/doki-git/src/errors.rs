
#[derive(Debug)]
pub struct GitError {
    pub kind: Box<GitErrorKind>,
}
#[derive(Debug)]
pub enum GitErrorKind {
    UnknownError,
    GitError(String),
}

pub type Result<T> = std::result::Result<T, GitError>;


impl From<git2::Error> for GitError {
    fn from(e: git2::Error) -> Self {
        Self {
            kind: box GitErrorKind::GitError(e.message().to_string())
        }
    }
}


impl From<()> for GitError {
    fn from(_: ()) -> Self {
        Self {
            kind: box GitErrorKind::UnknownError
        }
    }
}