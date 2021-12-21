#[test]
fn ready() {
    println!("it works!")
}

use serde::{Serialize, Deserialize, Deserializer, de};
use serde::de::{Visitor, SeqAccess};
use std::fmt::Formatter;
use serde::de::value::SeqAccessDeserializer;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct DokiConfig {
    /// base url
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "parse_base_url", default)]
    base_url: Vec<String>,
}

fn parse_base_url<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where D: Deserializer<'de>
{
    struct Accepted;

    impl<'de> Visitor<'de> for Accepted {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("base_url: Null | String | List<String>")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where E: de::Error
        {
            Ok(vec![s.to_owned()])
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
        {
            Ok(vec![])
        }

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
            where S: SeqAccess<'de>
        {
            Deserialize::deserialize(SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(Accepted)
}

#[test]
fn config() {
    let config = "
{
    base_url: 1,
    // A traditional message.
    message: 'hello world',

    // A number for some reason.
    n: 42,
}
";

    assert_eq!(
        json5::from_str(config),
        Ok(DokiConfig {
            base_url: vec![String::from("a")],
        }),
    );
}