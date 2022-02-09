use super::*;

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
}

#[test]
fn test_version() {
    let cfg = load_config_string(include_str!("version.json5"), FileFormat::Json5);
    println!("{:#?}", DokiVersion::parse(cfg.cache));
}