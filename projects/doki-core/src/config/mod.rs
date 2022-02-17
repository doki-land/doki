mod languages;
mod mode;
mod navbar;
#[cfg(feature = "non-wasm")]
mod parsing;
mod path;
mod sidebar;
#[cfg(test)]
mod test;
mod version;

use self::mode::DokiUrlMode;
pub use self::{languages::DokiLanguages, navbar::*, path::DokiPath, sidebar::DokiSidebar, version::DokiVersion};
use crate::ComponentData;
use doki_error::{DokiError, Result, Url};
use fs::read_dir;
use log::{error, info};
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::{Debug, Display, Formatter, Write},
    fs,
    fs::DirEntry,
    lazy::SyncLazy,
    path::{Path, PathBuf},
};

#[cfg(feature = "non-wasm")]
use {self::parsing::*, config::*};

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

impl Default for DokiConfig {
    fn default() -> Self {
        Self { url_base: vec![], url_end: "".to_string(), version: Default::default(), i18n: Default::default() }
    }
}

impl<'a> ComponentData<'a> for DokiSidebar {}

pub struct DokiDatabase {}
