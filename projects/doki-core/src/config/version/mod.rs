use super::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DokiVersionControl {
    pub enable: bool,
    #[serde(skip_serializing_if = "is_default")]
    #[serde(deserialize_with = "parse_as_lowercase_string", default)]
    pub head: String,
}

impl Default for DokiVersionControl {
    fn default() -> Self {
        Self { enable: false, head: "nightly".to_string() }
    }
}

impl DokiConfig {}
