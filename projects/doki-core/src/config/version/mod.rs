use super::*;



#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DokiVersionControl {
    enable: bool
}

impl DokiConfig {
    pub fn version_enable(&self) -> bool {
        match &self.version {
            None => {false}
            Some(s) => {s.enable}
        }
    }
}
