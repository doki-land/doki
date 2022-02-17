use super::*;

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

impl DokiUrlMode {
    pub fn rewrite_url(&self, url: Url, path: Vec<String>) -> Result<Url> {
        match self {
            // do nothing
            Self::HtmlData => Ok(url),
            Self::UrlPath => Ok(url.join(&path.join("/"))?),
            Self::UrlQuery(name) => {
                let mut out = url;
                out.query_pairs_mut().append_pair(name, &path.join("-"));
                Ok(out)
            }
            // TODO: url.domain()
            Self::SubDomain => Err(DokiError::runtime_error("unimplemented: sub domain resolve for version")),
        }
    }
}

pub fn rewrite_url() {

}

pub fn rewrite_path() {

}

impl DokiSidebar {
    pub fn get_link() {

    }
}