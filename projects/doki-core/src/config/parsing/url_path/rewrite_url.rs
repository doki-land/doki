use super::*;
use crate::config::{
    mode::LOCALHOST,
    sidebar::{SidebarGroup, SidebarGroupItemKind, SidebarItem, SidebarList},
};

#[inline]
fn rewrite_url_query(url: &Url, key: &str, value: &str) -> Result<Url> {
    let mut out = url.to_owned();
    out.query_pairs_mut().append_pair(key, value);
    Ok(out)
}

#[inline]
fn rewrite_url_dir(url: &Url, path: &str) -> Result<Url> {
    match path.is_empty() {
        true => { Ok(url.to_owned()) }
        false => Ok(url.join(&format!("{}/", path))?)
    }
}

#[inline]
fn rewrite_url_list(url: &Url, path: &Vec<String>) -> Result<Url> {
    let mut path = path.clone();
    path.push(String::new());
    Ok(url.join(&path.join("/"))?)
}

#[inline]
fn rewrite_url_list_maybe(url: &Url, list: &Option<Vec<String>>, one: &str) -> Result<Url> {
    match list {
        None if one.is_empty() => { Ok(url.to_owned()) }
        None => { rewrite_url_dir(url, one) }
        Some(s) => { rewrite_url_list(url, s) }
    }
}

impl DokiConfig {
    pub fn get_link(&self) -> Result<Url> {
        let out = rewrite_url_list(&LOCALHOST, &self.url_base)?;
        Ok(out)
    }
}

impl DokiLanguages {
    pub fn get_link(&self, base: &Url, language: &str) -> Result<Url> {
        match self.mode {
            // do nothing
            DokiUrlMode::HtmlData => Ok(base.to_owned()),
            DokiUrlMode::UrlPath => Ok(base.join(language)?),
            DokiUrlMode::UrlQuery { short } => {
                let name = match short {
                    true => "l",
                    false => "language",
                };
                rewrite_url_query(base, name, language)
            }
            // TODO: url.domain()
            DokiUrlMode::SubDomain => Err(DokiError::runtime_error("unimplemented: sub domain resolve for version")),
        }
    }
}

impl DokiVersion {
    pub fn get_link(&self, base: &Url, version: &str) -> Result<Url> {
        match self.mode {
            // do nothing
            DokiUrlMode::HtmlData => Ok(base.to_owned()),
            DokiUrlMode::UrlPath => Ok(base.join(version)?),
            DokiUrlMode::UrlQuery { short } => {
                let name = match short {
                    true => "v",
                    false => "version",
                };
                rewrite_url_query(base, name, version)
            }
            // TODO: url.domain()
            DokiUrlMode::SubDomain => Err(DokiError::runtime_error("unimplemented: sub domain resolve for version")),
        }
    }
}

impl DokiSidebar {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        match &self.url {
            Some(s) => Ok(base.join(s)?),
            _ => Ok(base.join(&self.section)?),
        }
    }
}

impl SidebarGroup {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        rewrite_url_list_maybe(base, &self.rewrite_url, "")
    }
}

impl SidebarGroupItemKind {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        match self {
            Self::Simple(item) => item.get_link(base),
            Self::List(item) => item.get_link(base),
        }
    }
}

impl SidebarList {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        rewrite_url_list_maybe(base, &self.rewrite_url, &self.title)
    }
}

impl SidebarItem {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        let out = match &self.url {
            Some(s) => base.join(&s.join("/"))?,
            _ => base.join(&self.name)?,
        };
        Ok(out)
    }
    pub fn get_link_relative(&self, base: &Url, end: &str) -> Result<String> {
        let url = self.get_link(base)?;
        let out = match end.is_empty() {
            true => url,
            false => url.join(end)?,
        };
        match LOCALHOST.make_relative(&out) {
            Some(s) => Ok(s),
            None => Err(DokiError::runtime_error("unsolved relative url")),
        }
    }
}
