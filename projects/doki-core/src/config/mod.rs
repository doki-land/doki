mod languages;
mod mode;
mod parsing;
mod path;
mod sidebar;
#[cfg(test)]
mod test;
mod version;
pub use self::{languages::DokiLanguages, path::DokiPath, version::DokiVersion};
use self::{mode::DokiUrlMode, parsing::*};
use config::{FileFormat, Value};
use doki_error::{DokiError, Result, Url};
use std::{collections::HashMap, fmt::Write};

#[derive(Debug, Clone, PartialEq)]
pub struct DokiConfig {
    /// base url
    pub url_base: Vec<String>,
    /// end of url
    /// - '/'
    /// - '.ext': e.g. '.html', '.php'
    pub url_end: String,
    /// [`DokiVersionControl`]
    pub version: DokiVersion,
    /// [`DokiInternationalization`]
    pub i18n: DokiLanguages,
}
