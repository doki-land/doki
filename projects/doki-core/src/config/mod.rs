mod languages;
mod mode;
mod parsing;
mod path;
#[cfg(test)]
mod test;
mod version;
mod sidebar;

pub use self::{languages::DokiLanguages, path::DokiPath, version::DokiVersion};

use self::parsing::*;
use config::{Value,FileFormat};
use std::fmt::{Write};
use std::{collections::HashMap};
use self::mode::DokiUrlMode;


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
