use super::*;

impl DokiUrlMode {
    #[inline]
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

#[inline]
fn rewrite_url_query(url: &Url, path: Vec<String>, name: &str) -> Result<Url> {
    let mut out = url;
    out.query_pairs_mut().append_pair(name, &path.join("-"));
    Ok(out)
}

#[inline]
pub fn rewrite_url(url: &Url, path: Vec<String>) -> Result<Url> {
    Ok(url.join(&path.join("/"))?)
}


impl DokiConfig {
    pub fn get_link(&self) -> Result<Url> {
        let root = Url::parse("/")?;
        rewrite_url(&root, self.get_url_path())
    }
}

impl DokiSidebar {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        rewrite_url(base, self.get_url_path())
    }
    #[inline]
    fn get_url_path(&self) -> Vec<String> {
        match &self.url {
            Some(s) => { vec![s.to_owned()] }
            _ => { vec![self.section.to_owned()] }
        }
    }
}