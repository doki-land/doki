mod basic;
mod url_path;
pub use self::{basic::*, url_path::*};
use super::*;
use fs::read_to_string;
use log::warn;
use std::{fs, fs::canonicalize, path::Path};

pub fn absolute_path(dir: &Path) -> String {
    canonicalize(dir).unwrap_or_default().to_string_lossy().trim_start_matches("\\\\?\\").to_string()
}
