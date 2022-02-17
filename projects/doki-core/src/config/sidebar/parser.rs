use std::path::PathBuf;
use super::*;

impl DokiSidebar {
    pub fn parse(raw: Value) -> Self {
        let default = Self::default();
        let root = match raw.into_table() {
            Ok(o) => o,
            Err(_) => return default,
        };
        let enable = parse_bool(&root, "enable").unwrap_or(default.enable);
        let section = parse_string(&root, "section").unwrap_or(default.section);
        Self { enable, section, url: parse_string(&root, "url"), items: Self::parse_items(&root) }
    }
    #[inline]
    fn parse_items(root: &HashMap<String, Value>) -> Vec<SidebarGroup> {
        let items = match parse_array(root, "items") {
            None => return vec![],
            Some(s) => s,
        };
        items.into_iter().map(SidebarGroup::parse).collect()
    }
}

impl SidebarGroup {
    pub fn parse(root: Value) -> Self {
        let default = Self::default();
        let root = match root.into_table() {
            Ok(o) => o,
            Err(_) => return default,
        };
        let title = parse_string(&root, "group");
        Self { title, items: Self::parse_items(&root) }
    }
    #[inline]
    fn parse_items(root: &HashMap<String, Value>) -> Vec<SidebarGroupItemKind> {
        let mut out = vec![];
        if let Some(s) = parse_array(root, "items") {
            for i in s {
                if let Some(s) = SidebarGroupItemKind::parse(i) {
                    out.push(s)
                }
            }
        }
        out
    }
}

impl SidebarGroupItemKind {
    pub fn parse(root: Value) -> Option<Self> {
        let root = root.into_table().ok()?;
        if let Some(_) = root.get("name") {
            return Some(Self::Simple(SidebarItem::parse(root)));
        }
        if let Some(_) = root.get("list") {
            return Some(Self::List(SidebarList::parse(root)));
        }
        None
    }
}

impl SidebarItem {
    pub fn parse(root: Map<String, Value>) -> Self {
        let default = Self::default();
        let name = parse_string(&root, "name").unwrap_or(default.name);
        Self { name, icon: None, link: "".to_string() }
    }
}

impl SidebarList {
    pub fn parse(root: Map<String, Value>) -> Self {
        let default = Self::default();
        let title = parse_string(&root, "list").unwrap_or(default.title);
        Self { title, path: "".to_string(), url: "".to_string(), icon: None, foldable: false, folded: false, items: vec![] }
    }
}
