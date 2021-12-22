#![feature(box_syntax)]

mod errors;
mod github;
pub mod local;

pub use errors::{GitError, GitErrorKind, Result};
