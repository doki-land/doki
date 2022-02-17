mod languages;
mod mode;
mod parsing;
mod path;
mod sidebar;
#[cfg(test)]
mod test;
mod version;
mod navbar;

use fs::read_dir;
use log::{error, info};
use semver::Version;
use std::{
    collections::BTreeMap,
    fs,
    fs::{DirEntry},
    path::{Path, PathBuf},
};
use serde_derive::{Deserialize, Serialize};
pub use self::{languages::DokiLanguages, path::DokiPath, sidebar::DokiSidebar, version::DokiVersion,navbar::*};
use self::{mode::DokiUrlMode, parsing::*};
use doki_error::{DokiError, Result, Url};
use std::{collections::HashMap, fmt::Write};
#[cfg(feature = "non-wasm")]
use {
    config::{Config, File, FileFormat, Map, Value}
};
use crate::ComponentData;

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

impl<'a> ComponentData<'a> for DokiSidebar { }

pub struct DokiDatabase {}
