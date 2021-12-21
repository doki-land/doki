mod i18n;
mod parsing;
#[cfg(test)]
mod test;
mod version;
mod path;

use self::{
    i18n::DokiInternationalization,
    parsing::{parse_null_as_default, parse_url_base, parse_url_end},
    version::DokiVersionControl,
};
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
    url_base: Vec<String>,
    /// end of url
    /// - '/'
    /// - '.ext': e.g. '.html', '.php'
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "parse_url_end", default)]
    url_end: Option<String>,
    /// [`DokiVersionControl`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "parse_null_as_default", default)]
    version: Option<DokiVersionControl>,
    /// [`DokiInternationalization`]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "parse_null_as_default", default)]
    i18n: Option<DokiInternationalization>,
}
