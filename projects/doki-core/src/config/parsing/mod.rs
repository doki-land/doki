use config::{Config, File};
use super::*;

#[inline]
pub fn parse_bool(root: &HashMap<String, Value>, key: &str) -> Option<bool> {
    get_normalized_object(root, key)?.clone().into_bool().ok()
}

#[inline]
pub fn parse_string(root: &HashMap<String, Value>, key: &str) -> Option<String> {
    get_normalized_object(root, key)?.clone().into_string().ok()
}

#[inline]
pub fn parse_object(root: &HashMap<String, Value>, key: &str) -> Option<HashMap<String, Value>> {
    get_normalized_object(root, key)?.clone().into_table().ok()
}

#[inline]
pub fn parse_array(root: &HashMap<String, Value>, key: &str) -> Option<Vec<Value>> {
    get_normalized_object(root, key)?.clone().into_array().ok()
}

pub fn parse_string_list(root: &HashMap<String, Value>, key: &str) -> Option<Vec<String>> {
    let head = get_normalized_object(root, key)?;
    if let Ok(o) = head.clone().into_string() {
        return Some(vec![o]);
    }
    let mut array = vec![];
    for i in head.clone().into_array().ok()? {
        array.push(i.into_string().ok()?)
    }
    Some(array)
}

fn get_normalized_object<'a>(root: &'a HashMap<String, Value>, key: &str) -> Option<&'a Value> {
    let target = normalized_string(key);
    for (key, v) in root {
        if normalized_string(key) == target {
            return Some(v);
        }
    }
    None
}

pub fn normalized_string(key: &str) -> String {
    let mut normalized = String::new();
    for c in key.to_ascii_lowercase().chars() {
        if c.is_ascii_alphanumeric() { normalized.push(c) } else { normalized.push('_') }
    }
    normalized
}

pub fn load_config_string(input: &str, format: FileFormat) -> Config {
    Config::builder()
        .add_source(File::from_str(input, format))
        .build()
        .unwrap()
}