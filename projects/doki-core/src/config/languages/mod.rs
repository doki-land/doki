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

impl DokiLanguages {
    pub fn write_url(&self, url: &mut Url, path: &str) -> Result<()> {
        match self.mode {
            DokiUrlMode::HtmlData => {
                // do nothing
                return Err(DokiError::runtime_error("unimplemented!"));
            }
            DokiUrlMode::UrlPath => *url = url.join(path)?,
            DokiUrlMode::UrlParameter { short } => {
                match short {
                    true => url.query_pairs_mut().append_pair("l", path),
                    false => url.query_pairs_mut().append_pair("language", path),
                };
            }
            DokiUrlMode::SubDomain => {
                // TODO: url.domain()
                return Err(DokiError::runtime_error("unimplemented: sub domain resolve for version"));
            }
        }
        Ok(())
    }
}

#[test]
fn test_language() {
    let cfg = load_config_string(include_str!("language.json5"), FileFormat::Json5);
    println!("{:#?}", DokiLanguages::parse(cfg.cache));
}
