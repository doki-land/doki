#[cfg(feature = "non-wasm")]
mod parser;

mod url_builder;

use super::*;

pub static LOCALHOST: SyncLazy<Url> = SyncLazy::new(|| Url::parse("https://localhost").unwrap());

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DokiUrlMode {
    HtmlData,
    UrlPath,
    UrlQuery { short: bool },
    SubDomain,
}

pub struct UrlBuilder {
    query: HashMap<String, String>,
    end: String,
}


impl Default for DokiUrlMode {
    fn default() -> Self {
        Self::UrlQuery { short: false }
    }
}
