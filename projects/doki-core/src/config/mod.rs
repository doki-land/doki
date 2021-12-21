mod i18n;
mod parsing;
mod path;
#[cfg(test)]
mod test;
mod version;

pub use self::{i18n::DokiInternationalization, path::DokiPath, version::DokiVersionControl};

use self::parsing::{is_default,parse_null_as_default, parse_url_base, parse_url_end};
use serde::{
    de,
    de::{value::SeqAccessDeserializer, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt::{Formatter, Write};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DokiConfig {
    /// base url
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "parse_url_base", default)]
    pub url_base: Vec<String>,
    /// end of url
    /// - '/'
    /// - '.ext': e.g. '.html', '.php'
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(deserialize_with = "parse_url_end", default)]
    pub url_end: String,
    /// [`DokiVersionControl`]
    #[serde(skip_serializing_if = "is_default")]
    #[serde(deserialize_with = "parse_null_as_default", default)]
    pub version: DokiVersionControl,
    /// [`DokiInternationalization`]
    #[serde(skip_serializing_if = "is_default")]
    #[serde(deserialize_with = "parse_null_as_default", default)]
    pub i18n: DokiInternationalization,
}
