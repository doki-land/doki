use crate::{DokiError, DokiErrorKind};
use bincode::ErrorKind;
use serde_json::Error;
use yggdrasil_shared::DiagnosticLevel;

impl From<Error> for DokiError {
    fn from(e: Error) -> Self {
        DokiError::syntax_error(e.to_string())
    }
}

impl From<Box<ErrorKind>> for DokiError {
    fn from(e: Box<ErrorKind>) -> Self {
        let kind = match *e {
            ErrorKind::Io(i) => DokiErrorKind::IOError(i),
            ErrorKind::InvalidUtf8Encoding(e) => DokiErrorKind::SyntaxError(e.to_string()),
            ErrorKind::InvalidBoolEncoding(e) => DokiErrorKind::SyntaxError(e.to_string()),
            ErrorKind::InvalidCharEncoding => DokiErrorKind::SyntaxError("Invalid char".to_string()),
            ErrorKind::InvalidTagEncoding(e) => DokiErrorKind::SyntaxError(e.to_string()),
            ErrorKind::DeserializeAnyNotSupported => DokiErrorKind::RuntimeError("Deserialize failed".to_string()),
            ErrorKind::SizeLimit => DokiErrorKind::RuntimeError("Size limit".to_string()),
            ErrorKind::SequenceMustHaveLength => DokiErrorKind::RuntimeError("Sequence must have length".to_string()),
            ErrorKind::Custom(e) => DokiErrorKind::RuntimeError(e),
        };
        Self { kind: Box::new(kind), level: DiagnosticLevel::None, file: None, range: None }
    }
}
