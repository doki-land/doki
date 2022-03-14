#![forbid(missing_docs)]
#![doc = include_str!("../readme.md")]
#![allow(clippy::needless_return)]

mod error;
mod error_3rd;

pub use self::error::{DokiError, DokiErrorKind, MaybeRanged, Result};
pub use url::Url;
pub use yggdrasil_shared::DiagnosticLevel;
