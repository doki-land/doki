mod parsing;
mod version;
mod i18n;
#[cfg(test)]
mod test;

use serde::{Serialize, Deserialize, Deserializer, de};
use serde::de::{Visitor, SeqAccess};
use std::fmt::Formatter;
use serde::de::value::SeqAccessDeserializer;
use self::parsing::{parse_url_base, parse_url_end, parse_null_as_default};
use self::version::DokiVersionControl;
use self::i18n::DokiInternationalization;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DokiConfig {
    /// base url
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "parse_url_base", default)]
    url_base: Vec<String>,
    /// end of url
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "parse_url_end", default)]
    url_end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "parse_null_as_default", default)]
    version: Option<DokiVersionControl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "parse_null_as_default", default)]
    i18n: Option<DokiInternationalization>,
}

impl DokiConfig {
    pub fn build_url(&self, path: &[String]) -> String {
        let mut out: String = path.join("/");
        match &self.url_end {
            None => {}
            Some(s) => {
                out.push_str(s)
            }
        }
        return out;
    }
}
