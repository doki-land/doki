use super::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DokiInternationalization {
    pub enable: bool,
}

impl Default for DokiInternationalization {
    fn default() -> Self {
        Self { enable: false }
    }
}

impl DokiConfig {

}
