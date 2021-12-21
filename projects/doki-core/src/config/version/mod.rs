use super::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DokiVersionControl {
    pub enable: bool,
}

impl Default for DokiVersionControl {
    fn default() -> Self {
        Self { enable: false }
    }
}

impl DokiConfig {

}
