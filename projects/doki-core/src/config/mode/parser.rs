use super::*;

impl DokiUrlMode {
    pub fn parse(root: &HashMap<String, Value>, key: &str, query: (&'static str, &'static str)) -> Option<Self> {
        if let Some(o) = parse_string(root, key) {
            return match safe_url_string(&o).as_str() {
                "url" => Some(Self::UrlQuery(query.0.to_string())),
                "url-short" => Some(Self::UrlQuery(query.1.to_string())),
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
