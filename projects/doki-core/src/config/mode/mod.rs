#[cfg(feature = "non-wasm")]
mod parser;

use super::*;



#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DokiUrlMode {
    HtmlData,
    UrlPath,
    UrlParameter { short: bool },
    SubDomain,
}

impl Default for DokiUrlMode {
    fn default() -> Self {
        Self::UrlParameter { short: true }
    }
}

