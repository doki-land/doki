#[cfg(feature = "non-wasm")]
mod parser;

mod path_builder;
mod url_builder;

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DokiUrlMode {
    HtmlData,
    UrlPath,
    UrlQuery { short: bool },
    SubDomain,
}

#[derive(Clone, PartialEq)]
pub struct UrlBuilder {
    domain: Vec<String>,
    host: String,
    path: Vec<String>,
    query: BTreeMap<String, String>,
    end: String,
}

impl Default for DokiUrlMode {
    fn default() -> Self {
        Self::UrlQuery { short: false }
    }
}
