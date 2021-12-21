use super::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DokiInternationalization {
    enable: bool,
}

impl Default for DokiInternationalization {
    fn default() -> Self {
        Self { enable: false }
    }
}

impl DokiConfig {
    pub fn i18n_enable(&self) -> bool {
        match &self.i18n {
            None => false,
            Some(s) => s.enable,
        }
    }
}
