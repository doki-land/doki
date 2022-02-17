mod display;
#[cfg(feature = "non-wasm")]
mod parser;
#[cfg(test)]
mod test;

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DokiSidebar {
    /// show this sidebar or not
    pub enable: bool,
    /// section name of page sidebar
    pub section: String,
    /// set specialized url segment
    ///
    /// use [`section`] if missing
    pub url: Option<String>,
    /// groups in this sidebar
    pub items: Vec<SidebarGroup>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SidebarGroup {
    /// title of the sidebar group
    pub title: Option<String>,
    pub rewrite_url: Option<Vec<String>>,
    pub items: Vec<SidebarGroupItemKind>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SidebarGroupItemKind {
    /// A sidebar item
    Simple(SidebarItem),
    /// a sidebar list
    List(SidebarList),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SidebarItem {
    /// title of the item
    pub name: String,
    /// icon of the item
    pub icon: Option<SidebarItemIcon>,
    pub path: Option<String>,
    pub url: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SidebarList {
    /// title of list
    pub title: String,
    /// unix relative path
    pub rewrite_path: Option<String>,
    /// rewrite url
    pub rewrite_url: Option<Vec<String>>,
    /// icon of the list
    pub icon: Option<SidebarItemIcon>,
    /// This tab can be collapsed
    pub foldable: bool,
    ///
    pub folded: bool,
    /// items of the list
    pub items: Vec<SidebarItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SidebarItemIcon {
    Numeric(Vec<usize>),
    IconSVG(String),
    IconLink(String),
}
