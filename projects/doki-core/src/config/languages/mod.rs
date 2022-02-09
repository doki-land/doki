use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DokiLanguages {
    pub enable: bool,
    pub base: String,
}

impl Default for DokiLanguages {
    fn default() -> Self {
        Self { enable: false, base: "en".to_string() }
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
        // let head = parse_string_list(&root, "head").unwrap_or(default.head);
        Self { enable, base: "".to_string() }
    }
}
