use std::collections::HashMap;
use config::Value;
use super::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DokiVersion {
    pub enable: bool,
    pub head: Vec<String>,
}

impl Default for DokiVersion {
    fn default() -> Self {
        Self { enable: false, head: vec![String::from("latest")] }
    }
}

impl DokiVersion {
    pub fn parse(raw: Value) -> Self {
        let default = Self::default();
        let root = raw.into_table()?;
        let enable = Self::parse_enable(&root).unwrap_or(default.enable);
        let head = Self::parse_head(&root).unwrap_or(default.head);
        Self {
            enable,
            head,
        }
    }
    #[inline]
    fn parse_enable(root: &HashMap<String, Value>) -> Option<bool> {
        root.get("enable")?.into_bool().ok()
    }

    fn parse_head(root: &HashMap<String, Value>) -> Option<Vec<String>> {
        let head = root.get("head")?;
        if let Ok(o) = head.into_str() {
            return Some(vec![o]);
        }
        let mut array = vec![];
        for i in head.clone().into_array()? {
            array.push(i.into_str()?)
        }
        Some(array)
    }
}
