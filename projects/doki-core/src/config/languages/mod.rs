use super::*;
#[cfg(feature = "non-wasm")]
mod parser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DokiLanguages {
    pub enable: bool,
    pub mode: DokiUrlMode,
    pub base: String,
}

impl Default for DokiLanguages {
    fn default() -> Self {
        Self { enable: true, mode: Default::default(), base: "en".to_string() }
    }
}

impl DokiLanguages {}

#[test]
fn test_language() {
    let cfg = load_config_string(include_str!("language.json5"), FileFormat::Json5);
    println!("{:#?}", DokiLanguages::parse(cfg.cache));
}
