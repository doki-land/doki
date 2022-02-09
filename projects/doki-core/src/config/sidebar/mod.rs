mod display;
#[cfg(feature = "non-wasm")]
mod parser;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct DokiSidebar {
    /// show this sidebar or not
    pub enable: bool,
    /// section name of page sidebar
    pub section: String,
    /// set specialized url segment
    ///
    /// use [`DokiSidebar::section`] if missing
    url: Option<String>,
    /// groups in this sidebar
    pub items: Vec<SidebarGroup>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SidebarGroup {
    /// title of the sidebar group
    pub title: Option<String>,
    pub items: Vec<SidebarGroupItemKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SidebarGroupItemKind {
    /// A sidebar item
    Simple(SidebarItem),
    /// a sidebar list
    List(SidebarList),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SidebarItem {
    /// title of the item
    pub name: String,
    /// icon of the item
    pub icon: Option<SidebarItemIcon>,
    pub link: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SidebarList {
    /// title of list
    pub title: String,
    /// unix path
    pub path: String,

    url: String,
    /// icon of the list
    pub icon: Option<SidebarItemIcon>,
    /// This tab can be collapsed
    pub foldable: bool,
    ///
    pub folded: bool,
    /// items of the list
    pub items: Vec<SidebarItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SidebarItemIcon {
    Numeric(Vec<usize>),
    Icon(String),
}

impl DokiSidebar {
    /// get current segment of url
    pub fn get_url_segment(&self) -> String {
        let url = self.url.as_ref().unwrap_or(&self.section);
        normalized_string(url)
    }
    pub fn write_url(&self, url: &mut Url) -> Result<()> {
        *url = url.join(&self.get_url_segment())?;
        Ok(())
    }
}

#[test]
fn test_sidebar() {
    let cfg = load_config_string(include_str!("sidebar.json5"), FileFormat::Json5);
    println!("{:#?}", DokiSidebar::parse(cfg.cache));
}
