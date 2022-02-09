use super::*;
use fs::read_to_string;
use log::warn;
use std::{fs, path::Path};

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
    if key.chars().all(|c| c.is_ascii_alphanumeric()) {
        return root.get(key);
    }
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
        if c.is_ascii_alphanumeric() { normalized.push(c) } else { normalized.push('-') }
    }
    normalized
}

pub fn load_config_string(input: &str, format: FileFormat) -> Config {
    Config::builder().add_source(File::from_str(input, format)).build().unwrap()
}

pub fn load_config_file(dir: &Path, name: &str) -> Result<Value> {
    let mut config = Config::builder();
    let mut loaded = 0;

    let yaml = dir.join(format!("{}.yaml", name));
    if let Ok(o) = read_to_string(yaml) {
        config = config.add_source(File::from_str(&o, FileFormat::Yaml));
        loaded += 1
    }

    let toml = dir.join(format!("{}.toml", name));
    if let Ok(o) = read_to_string(toml) {
        config = config.add_source(File::from_str(&o, FileFormat::Toml));
    }

    let json = dir.join(format!("{}.json", name));
    if let Ok(o) = read_to_string(json) {
        config = config.add_source(File::from_str(&o, FileFormat::Json));
    }

    let json5 = dir.join(format!("{}.json5", name));
    match read_to_string(json5) {
        Ok(o) => config = config.add_source(File::from_str(&o, FileFormat::Json5)),
        Err(..) => {
            warn!("The configuration about {name} was not found in {dir:?},", name = name, dir = dir);
            warn!("which means {dir:?} may not be a valid directory.", dir = dir);
        }
    }
    // FIXME: unwrap
    Ok(config.build().unwrap().cache)
}
