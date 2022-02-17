mod basic;
mod url_path;
use super::*;
use fs::read_to_string;
use log::warn;
use std::{fs, fs::canonicalize, path::Path};
pub use self::{url_path::*, basic::*};

pub fn absolute_path(dir: &Path) -> String {
    canonicalize(dir).unwrap_or_default().to_string_lossy().trim_start_matches("\\\\?\\").to_string()
}
