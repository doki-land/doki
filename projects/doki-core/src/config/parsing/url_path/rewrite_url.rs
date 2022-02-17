use super::*;
use crate::config::{
    mode::UrlBuilder,
    sidebar::{SidebarGroup, SidebarGroupItemKind, SidebarItem, SidebarList},
};

impl DokiConfig {
    pub fn get_link(&self) -> UrlBuilder {
        let mut url = UrlBuilder::default();
        url.path_extend(&self.url_base);
        url
    }
    pub fn set_link_end(&self, base: &mut UrlBuilder) {
        base.set_extension(&self.url_end)
    }
}

impl DokiLanguages {
    pub fn get_link(&self, base: &UrlBuilder, language: &str) -> UrlBuilder {
        let mut out = base.clone();
        match self.mode {
            DokiUrlMode::HtmlData => {}
            DokiUrlMode::UrlPath => out.path_join(language),
            DokiUrlMode::UrlQuery { short } => {
                let name = match short {
                    true => "l",
                    false => "language",
                };
                out.query_insert(name, language)
            }
            DokiUrlMode::SubDomain => out.domain_join(language),
        }
        out
    }
}

impl DokiVersion {
    pub fn get_link(&self, base: &UrlBuilder, version: &str) -> UrlBuilder {
        let mut out = base.clone();
        match self.mode {
            DokiUrlMode::HtmlData => {}
            DokiUrlMode::UrlPath => out.path_join(version),
            DokiUrlMode::UrlQuery { short } => {
                let name = match short {
                    true => "v",
                    false => "version",
                };
                out.query_insert(name, version)
            }
            DokiUrlMode::SubDomain => out.domain_join(version),
        }
        out
    }
}

impl DokiSidebar {
    pub fn get_link(&self, base: &UrlBuilder) -> UrlBuilder {
        debug_assert!(!self.section.is_empty(), "Missing section in DokiSidebar");
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
        // debug_assert!(!self.title.is_empty(), "Missing title in SidebarGroup");
        let mut out = base.clone();
        match &self.rewrite_url {
            None => {}
            Some(s) => out.path_extend(s),
        }
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
        debug_assert!(!self.title.is_empty(), "Missing title in SidebarList");
        let mut out = base.clone();
        match &self.rewrite_url {
            None => out.path_join(&self.title),
            Some(s) => out.path_extend(s),
        }
        out
    }
}

impl SidebarItem {
    pub fn get_link(&self, base: &UrlBuilder) -> UrlBuilder {
        debug_assert!(!self.name.is_empty(), "Missing name in SidebarItem");
        let mut out = base.clone();
        match &self.url {
            Some(s) => out.path_extend(s),
            None => out.path_join(&self.name),
        };
        out
    }
}
