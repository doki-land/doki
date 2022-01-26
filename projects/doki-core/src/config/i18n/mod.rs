use super::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DokiLocalization {
    pub enable: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(deserialize_with = "parse_as_lowercase_string", default)]
    pub main: String,
}

impl Default for DokiLocalization {
    fn default() -> Self {
        Self { enable: false, main: "en".to_string() }
    }
}

impl DokiConfig {}
