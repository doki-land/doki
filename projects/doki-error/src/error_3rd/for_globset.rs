use crate::DokiError;
use globset::Error;

impl From<Error> for DokiError {
    fn from(e: Error) -> Self {
        DokiError::runtime_error(e.to_string())
    }
}
