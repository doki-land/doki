use super::*;
use crate::config::{
    mode::LOCALHOST,
    sidebar::{SidebarGroup, SidebarGroupItemKind, SidebarItem, SidebarList},
};

macro_rules! rewrite_url {
    ($list:expr, $base:expr) => {
        match &$list {
            None => Ok(format!("{}/", $base)),
            Some(url) => rewrite_url($base, url),
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
fn rewrite_url_dir(url: &Url, path: &str) -> Result<Url> {
    Ok(url.join(&format!("{}/", path))?)
}

#[inline]
fn rewrite_url_list(url: &Url, path: &Vec<String>) -> Result<Url> {
    let mut path = path.clone();
    path.push(String::new());
    Ok(url.join(&path.join("/"))?)
}

#[inline]
fn rewrite_url_list_maybe(url: &Url, path: Option<Vec<String>>) -> Result<Url> {
    Ok(url.join(&path.join("/"))?)
}

impl DokiConfig {
    pub fn get_link(&self) -> Result<Url> {
        let out = rewrite_url_list(&LOCALHOST, &self.url_base)?;
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
                    true => "l",
                    false => "language",
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
                    true => "v",
                    false => "version",
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
        println!("{:#?}", base.to_string());
        println!("{:#?}", base.join("/a/").unwrap().to_string());


        match &self.url {
            Some(s) => Ok(base.join(s)?),
            _ => Ok(base.join(&self.section)?),
        }
    }
}

impl SidebarGroup {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        rewrite_url!(self.rewrite_url, base)
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
        rewrite_url!(self.rewrite_url, base)
    }
}

impl SidebarItem {
    pub fn get_link(&self, base: &Url) -> Result<Url> {
        match &self.url {
            Some(s) => rewrite_url_list(base, s),
            _ => Ok(base.join(&self.name)?),
        }
    }
    pub fn get_link_relative(&self, base: &Url, end: &str) -> Result<String> {
        let url = self.get_link(base)?;
        let out = match end.is_empty() {
            true => url,
            false => url.join(end)?,
        };
        match out.make_relative(&LOCALHOST) {
            Some(s) => Ok(s),
            None => Err(DokiError::runtime_error("unsolved relative url")),
        }
    }
}
