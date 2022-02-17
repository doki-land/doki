use super::*;

impl DokiLanguages {
    pub fn parse(raw: Value) -> Self {
        let default = Self::default();
        let root = match raw.into_table() {
            Ok(o) => o,
            Err(_) => return default,
        };
        let enable = parse_bool(&root, "enable").unwrap_or(default.enable);
        let base = parse_string(&root, "base").unwrap_or(default.base);
        Self { enable, mode: Default::default(), base }
    }
}
