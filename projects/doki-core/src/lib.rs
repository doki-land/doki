mod config;
mod errors;

mod backend_dioxus;
pub mod pages;

pub use config::{DokiConfig, DokiPath};
pub use errors::{DokiError, Result};
