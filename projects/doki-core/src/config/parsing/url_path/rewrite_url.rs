use super::*;
use crate::config::{
    mode::LOCALHOST,
    sidebar::{SidebarGroup, SidebarGroupItemKind, SidebarItem, SidebarList},
};
use crate::config::mode::UrlBuilder;

impl DokiConfig {
    pub fn get_link(&self) -> UrlBuilder {
        let mut url = UrlBuilder::default();
        url.path_extend(&self.url_base);
        url
    }
}

impl DokiLanguages {
    pub fn get_link(&self, base: &UrlBuilder, language: &str) -> UrlBuilder {
        let mut out = base.clone();
        match self.mode {
            // do nothing
            DokiUrlMode::HtmlData => {  },
            DokiUrlMode::UrlPath => {
                out.path_join(version)
            },
            DokiUrlMode::UrlQuery { short } => {
                let name = match short {
                    true => "l",
                    false => "language",
                };
                out.query_insert(name, language)
            }
            DokiUrlMode::SubDomain => {
                out.domain_join(language)
            },
        }
        out
    }
}

impl DokiVersion {
    pub fn get_link(&self, base: &UrlBuilder, version: &str) -> UrlBuilder {
        let mut out = base.clone();
        match self.mode {
            // do nothing
            DokiUrlMode::HtmlData => {},
            DokiUrlMode::UrlPath => {
                out.path_join(version)
            },
            DokiUrlMode::UrlQuery { short } => {
                let name = match short {
                    true => "v",
                    false => "version",
                };
                out.query_insert(name, version)
            }
            DokiUrlMode::SubDomain => {
                out.domain_join(version)
            },
        }
        out
    }
}

impl DokiSidebar {
    pub fn get_link(&self, base: &UrlBuilder) -> UrlBuilder {
        let mut out = base.clone();
        match &self.url {
            Some(s) => out.path_join(s),
            None => out.path_join(&self.section),
        }
        out
    }
}

impl SidebarGroup {
    pub fn get_link(&self, base: &UrlBuilder) -> UrlBuilder {
        let mut out = base.clone();
        out.path_extend(&self.rewrite_url.unwrap_or_default());
        out
    }
}

impl SidebarGroupItemKind {
    pub fn get_link(&self, base: &UrlBuilder) -> UrlBuilder {
        match self {
            Self::Simple(item) => item.get_link(base),
            Self::List(item) => item.get_link(base),
        }
    }
}

impl SidebarList {
    pub fn get_link(&self, base: &UrlBuilder) -> UrlBuilder {
        debug_assert_ne!(self.title.is_empty());
        let mut out = base.clone();
        match &self.rewrite_url {
            None => {out.path_join(&self.title)}
            Some(s) => {out.path_extend(s)}
        }
        out
    }
}

impl SidebarItem {
    pub fn get_link(&self, base: &UrlBuilder) -> UrlBuilder {
        debug_assert_ne!(self.name.is_empty());
        let mut out = base.clone();
        match &self.url {
            Some(s) => out.path_extend(s),
            None => out.path_join(&self.name),
        };
        out
    }
}
