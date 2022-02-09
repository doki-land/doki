use crate::TailwindError;
use git2::Error;

impl From<Error> for TailwindError {
    fn from(e: Error) -> Self {
        TailwindError::runtime_error(e.to_string())
    }
}
