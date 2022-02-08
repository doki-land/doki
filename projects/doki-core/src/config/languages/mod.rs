use super::*;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DokiLanguages {
    enable: bool,
    base: String,
}

impl Default for DokiLanguages {
    fn default() -> Self {
        Self { enable: false, base: "en".to_string() }
    }
}

impl DokiLanguages {
    pub fn parse(raw: Value) -> Self {
        let default = Self::default();
        let root = raw.into_table()?;
        let enable = parse_bool(&root, "enable").unwrap_or(default.enable);
        let head = parse_string_list(&root, "head").unwrap_or(default.head);
        Self {
            enable,
            head,
        }
    }
}
