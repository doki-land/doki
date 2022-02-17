use super::*;

impl DokiUrlMode {
    pub fn parse(root: &HashMap<String, Value>, key: &str) -> Option<Self> {
        if let Some(o) = parse_string(root, key) {
            return match safe_url_string(&o).as_str() {
                "url" => Some(Self::UrlParameter { short: false }),
                "url-short" => Some(Self::UrlParameter { short: true }),
                "path" | "url-path" => Some(Self::UrlPath),
                "html" => Some(Self::HtmlData),
                _ => None,
            };
        }
        let mode = parse_object(root, key)?;
        // TODO
        let _ = mode;
        None
    }
}
