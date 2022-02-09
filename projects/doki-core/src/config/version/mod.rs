use super::*;
use doki_error::DokiError;
use log::warn;
use semver::Version;
use std::{
    collections::BTreeMap,
    fs,
    fs::{DirEntry, ReadDir},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq)]
pub struct DokiVersion {
    pub enable: bool,
    pub mode: DokiUrlMode,
    pub head: Vec<String>,
}

impl Default for DokiVersion {
    fn default() -> Self {
        Self { enable: false, mode: Default::default(), head: vec![String::from("latest")] }
    }
}

impl DokiVersion {
    pub fn parse(raw: Value) -> Self {
        let default = Self::default();
        let root = match raw.into_table() {
            Ok(o) => o,
            Err(_) => return default,
        };
        let enable = parse_bool(&root, "enable").unwrap_or(default.enable);
        let head = parse_string_list(&root, "head").unwrap_or(default.head);
        let mode = DokiUrlMode::parse(&root, "mode").unwrap_or_default();
        Self { enable, mode, head }
    }

    pub fn write_url(&self, url: &mut Url, path: &str) -> Result<()> {
        match self.mode {
            DokiUrlMode::HtmlData => {}
            DokiUrlMode::UrlPath => *url = url.join(path)?,
            DokiUrlMode::UrlParameter { short } => {
                match short {
                    true => url.query_pairs_mut().append_pair("v", path),
                    false => url.query_pairs_mut().append_pair("version", path),
                };
            }
            DokiUrlMode::SubDomain => return Err(DokiError::runtime_error("Can not set sub domain for version!")),
        }
        Ok(())
    }
    pub fn load_directories(&self, dir: &Path) -> Result<Vec<(String, PathBuf)>> {
        let mut out = vec![];
        let mut vs = BTreeMap::default();

        for file in &self.head {
            let path = dir.join(&file);
            out.push((file.to_owned(), path))
        }
        for file in fs::read_dir(dir)? {
            let dir = match path_from_dir_result(file) {
                Some(s) => s,
                None => continue,
            };
            match parse_version_from_path(&dir) {
                Some(s) => {
                    vs.insert(s.0, (s.1, dir));
                }
                None => {
                    warn!("xx is not semver");
                    continue;
                }
            }
        }
        out.extend(vs.into_iter().map(|f| f.1));
        Ok(out)
    }
}

fn path_from_dir_result(res: std::io::Result<DirEntry>) -> Option<PathBuf> {
    let path = res.ok()?.path();
    if path.is_dir() {
        Some(path)
    }
    else {
        // warn!("xx is not dir");
        None
    }
}

fn parse_version_from_path(dir: &Path) -> Option<(Version, String)> {
    let name = dir.file_name()?.to_str()?;
    let mut collecting = false;
    let mut trimmed = String::new();
    for c in name.chars() {
        if collecting {
            trimmed.push(c)
        }
        else if c.is_numeric() {
            collecting = true;
            trimmed.push(c);
        }
        else {
            continue;
        }
    }
    if trimmed.is_empty() {
        return None;
    }
    let v = Version::parse(&trimmed).ok()?;
    Some((v, name.to_owned()))
}

#[test]
fn test_version() {
    let cfg = load_config_string(include_str!("version.json5"), FileFormat::Json5);
    println!("{:#?}", DokiVersion::parse(cfg.cache));
}
