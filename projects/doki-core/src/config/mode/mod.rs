#[cfg(feature = "non-wasm")]
mod parser;


use super::*;

pub static  LOCALHOST: SyncLazy<Url> = SyncLazy::new(|| Url::parse("https://localhost").unwrap());

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DokiUrlMode {
    HtmlData,
    UrlPath,
    UrlQuery {short:bool},
    SubDomain,
}

impl Default for DokiUrlMode {
    fn default() -> Self {
        Self::UrlQuery {  short: false }
    }
}
