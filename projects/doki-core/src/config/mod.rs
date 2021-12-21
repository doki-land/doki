mod parsing;
mod version;

use serde::{Serialize, Deserialize, Deserializer, de};
use serde::de::{Visitor, SeqAccess};
use std::fmt::Formatter;
use serde::de::value::SeqAccessDeserializer;
use self::parsing::parse_base_url;
use self::version::DokiVersionControl;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DokiConfig {
    /// base url
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "parse_base_url", default)]
    base_url: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    version: Option<DokiVersionControl>
}

impl DokiConfig {
    pub fn build_url(&self) {

    }
}

#[test]
fn config() {
    let config = "
{
    base_url: 'a/b',
    // A traditional message.
    message: 'hello world',

    // A number for some reason.
    n: 42,
}
";

    assert_eq!(
        json5::from_str(config),
        Ok(DokiConfig {
            base_url: vec![String::from("a"), String::from("b")],
            version: None
        }),
    );
}