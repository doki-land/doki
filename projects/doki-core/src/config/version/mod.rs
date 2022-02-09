use std::collections::HashMap;
use config::Value;
use crate::config::mode::DokiUrlMode;
use super::*;

#[derive(Debug, PartialEq)]
pub struct DokiVersion {
    pub enable: bool,
    pub mode: DokiUrlMode,
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
        let enable = parse_bool(&root, "enable").unwrap_or(default.enable);
        let head = parse_string_list(&root, "head").unwrap_or(default.head);
        Self {
            enable,
            head,
        }
    }
}
