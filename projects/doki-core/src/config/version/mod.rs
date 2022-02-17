#[cfg(feature = "non-wasm")]
mod parser;
use super::*;

#[cfg(feature = "non-wasm")]
pub use self::parser::load_version;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DokiVersion {
    pub enable: bool,
    pub mode: DokiUrlMode,
    pub head: Vec<String>,
    /// runtime only
    pub versions: Vec<(String, PathBuf)>,
}

impl Default for DokiVersion {
    fn default() -> Self {
        Self { enable: false, mode: Default::default(), head: vec![String::from("latest")], versions: vec![] }
    }
}

impl DokiVersion {
    fn load_directories(&mut self, dir: &Path) -> Result<()> {
        let dir_debug = absolute_path(dir);
        let mut vs = BTreeMap::default();

        for file in &self.head {
            let path = dir.join(&file);
            if path.exists() {
                self.versions.push((file.to_owned(), path))
            }
            else {
                error!("Version `{file}` not found in `{dir}`, `{file}` will be ignored.", file = file, dir = dir_debug)
            }
        }

        for file in read_dir(dir)? {
            let dir = match path_from_dir_result(file) {
                Some(s) => s,
                None => continue,
            };
            match parse_version_from_path(&dir) {
                Some(s) => {
                    vs.insert(s.0, (s.1, dir));
                }
                None => continue,
            }
        }
        self.versions.extend(vs.into_iter().map(|f| f.1).rev());
        if self.versions.is_empty() {
            info!("`{dir}` does not found any valid version directory.", dir = dir_debug)
        }
        Ok(())
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

#[test]
fn test_version_load() {
    env_logger::init();
    println!("{:#?}", load_version(&PathBuf::from("../doki-docs/posts/zh-hans")))
}
