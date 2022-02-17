use super::*;

pub(crate) fn parse_url(root: &HashMap<String, Value>) -> Option<String> {
    parse_string(&root, "url")
}

pub(crate) fn parse_url_fragment(root: &HashMap<String, Value>, key: &str) -> Option<Vec<String>> {
    let maybe_string = || -> Option<Vec<String>> {
        let url = parse_string(root, key)?;
        Some(url.split("/").map(safe_url_string).collect())
    };
    let maybe_array = || -> Option<Vec<String>> {
        let url = parse_string_list(root, key)?;
        Some(url.into_iter().map(|s| safe_url_string(&s)).collect())
    };
    maybe_string().or_else(maybe_array)
}

pub(crate) fn safe_url_string(key: &str) -> String {
    let mut normalized = String::new();
    for c in key.to_ascii_lowercase().chars() {
        if c.is_ascii_alphanumeric() { normalized.push(c) } else { normalized.push('-') }
    }
    normalized
}