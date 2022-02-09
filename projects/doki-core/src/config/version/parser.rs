use super::*;

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
        Self { enable, mode, head, ..Self::default() }
    }
}

pub fn load_version(dir: &Path) -> Result<DokiVersion> {
    let mut config = DokiVersion::parse(load_config_file(dir, "version")?);
    config.load_directories(dir)?;
    Ok(config)
}