use super::*;
use std::{collections::HashMap, str::FromStr};

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

impl DokiUrlMode {
    pub fn parse(root: &HashMap<String, Value>, key: &str) -> Option<Self> {
        if let Some(o) = parse_string(root, key) {
            return match normalized_string(&o).as_str() {
                "url" => Some(Self::UrlParameter { short: false }),
                "url_short" => Some(Self::UrlParameter { short: true }),
                "path" | "url_path" => Some(Self::UrlPath),
                "html" => Some(Self::HtmlData),
                _ => None,
            };
        }
        let mode = parse_object(root, key)?;
        // TODO
        let _ = mode;
        None
    }
}
