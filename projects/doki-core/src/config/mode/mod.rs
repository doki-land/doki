#[cfg(feature = "non-wasm")]
mod parser;

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DokiUrlMode {
    HtmlData,
    UrlPath,
    UrlQuery(String),
    SubDomain,
}

impl Default for DokiUrlMode {
    fn default() -> Self {
        Self::UrlQuery("query".to_string())
    }
}
