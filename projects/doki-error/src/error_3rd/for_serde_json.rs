use crate::DokiError;
use serde_json::Error;

impl From<Error> for DokiError {
    fn from(e: Error) -> Self {
        DokiError::syntax_error(e.to_string())
    }
}
