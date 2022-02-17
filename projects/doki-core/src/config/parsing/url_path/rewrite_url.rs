use crate::config::mode::LOCALHOST;
use crate::config::sidebar::{SidebarGroup, SidebarGroupItemKind, SidebarItem, SidebarList};
use super::*;

macro_rules! rewrite_url {
    ($list:expr, $base:expr) => {
        match &$list {
            None => { Ok($base.to_owned()) }
            Some(url) => { rewrite_url($base, url) }
        }
    };
}

#[inline]
fn rewrite_url_query(url: &Url, path: &str, name: &str) -> Result<Url> {
    let mut out = url.to_owned();
    out.query_pairs_mut().append_pair(name, path);
    Ok(out)
}

#[inline]
pub fn rewrite_url(url: &Url, path: &[String]) -> Result<Url> {
    Ok(url.join(&path.join("/"))?)
}




impl DokiConfig {
    pub fn get_link(&self) -> Result<Url> {
        let out = rewrite_url(&LOCALHOST, &self.url_base)?;
        Ok(out)
    }
}

impl DokiLanguages {
    pub fn get_link(&self, base: &Url, path: &str) -> Result<Url> {
        match self.mode {
            // do nothing
            DokiUrlMode::HtmlData => Ok(base.to_owned()),
            DokiUrlMode::UrlPath => Ok(base.join(path)?),
            DokiUrlMode::UrlQuery { short } => {
                let name = match short {
                    true => {"l"}
                    false => {"language"}
                };
                rewrite_url_query(base, path, name)
            }
            // TODO: url.domain()
            DokiUrlMode::SubDomain => Err(DokiError::runtime_error("unimplemented: sub domain resolve for version")),
        }
    }
}


impl DokiVersion {
    pub fn get_link(&self, base: &Url, path: &str) -> Result<Url> {
        match self.mode {
            // do nothing
            DokiUrlMode::HtmlData => Ok(base.to_owned()),
            DokiUrlMode::UrlPath => Ok(base.join(path)?),
            DokiUrlMode::UrlQuery { short } => {
                let name = match short {
                    true => {"v"}
                    false => {"version"}
                };
                rewrite_url_query(base, path, name)
            }
            // TODO: url.domain()
            DokiUrlMode::SubDomain => Err(DokiError::runtime_error("unimplemented: sub domain resolve for version")),
        }
    }
}

impl DokiSidebar {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        match &self.url {
            Some(s) => { Ok(base.join(s)?) }
            _ => { Ok(base.join(&self.section)?) }
        }
    }
}

impl SidebarGroup {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        rewrite_url!(self.rewrite_url ,base)
    }
}

impl SidebarGroupItemKind {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        match self {
            Self::Simple(item) => { item.get_link(base) }
            Self::List(item) => { item.get_link(base) }
        }
    }
}


impl SidebarList {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        rewrite_url!(self.rewrite_url ,base)
    }
}

impl SidebarItem {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        match &self.url {
            Some(s) => { rewrite_url(base, s) }
            _ => { Ok(base.join(&self.name)?) }
        }
    }
    pub fn get_link_relative(&self, base: &Url, end:&str) -> Result<String> {
        let url = self.get_link(base)?;
        let out = match end.is_empty() {
            true => {url}
            false => {
                url.join(end)?
            }
        };
        match out.make_relative(&LOCALHOST) {
            Some(s) => {Ok(s)},
            None => {Err(DokiError::runtime_error("unsolved relative url"))}
        }
    }
}
