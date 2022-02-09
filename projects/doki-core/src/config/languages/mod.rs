use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DokiLanguages {
    pub enable: bool,
    pub base: String,
}

impl Default for DokiLanguages {
    fn default() -> Self {
        Self { enable: true, base: "en".to_string() }
    }
}

impl DokiLanguages {
    pub fn parse(raw: Value) -> Self {
        let default = Self::default();
        let root = match raw.into_table() {
            Ok(o) => o,
            Err(_) => {
                return default;
            }
        };
        let enable = parse_bool(&root, "enable").unwrap_or(default.enable);
        let base = parse_string(&root, "base").unwrap_or(default.base);
        Self { enable, base }
    }
}


#[test]
fn test_language() {
    let cfg = load_config_string(include_str!("language.json5"), FileFormat::Json5);
    println!("{:#?}", DokiLanguages::parse(cfg.cache));
}