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
        Self { enable, section, url: parse_url(&root), items: Self::parse_items(&root) }
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
        let items = Self::parse_items(&root).unwrap_or(default.items);
        Self { title, items }
    }
    #[inline]
    fn parse_items(root: &HashMap<String, Value>) -> Option<Vec<SidebarGroupItemKind>> {
        let mut out = vec![];
        let s = parse_array(root, "items")?;
        for i in s {
            let item = SidebarGroupItemKind::parse(i)?;
            out.push(item)
        }
        Some(out)
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
        let link = Self::parse_link(&root).unwrap_or(default.link);
        Self { name, icon: None, url: None, link }
    }
}

impl SidebarList {
    pub fn parse(root: Map<String, Value>) -> Self {
        let default = Self::default();
        let title = parse_string(&root, "list").unwrap_or(default.title);
        Self { title, path: "".to_string(), url: "".to_string(), icon: None, foldable: false, folded: false, items: vec![] }
    }
}

pub(crate) fn parse_url(root: &HashMap<String, Value>) -> Option<String> {
    parse_string(&root, "url")
}

pub(crate) fn parse_url_fragment(root: &HashMap<String, Value>) -> Option<Vec<String>> {
    let maybe_string = || -> Option<Vec<String>> {
        let url = parse_string(root, "url")?;
        Some(url.split("/").collect())
    };
    let maybe_array = || -> Option<Vec<String>> {
        let url = parse_string_list(root, "url")?;
        Some(url)
    };
    let url = maybe_string().or_else(maybe_array)?;
    Some(url.into_iter().map(safe_url_string).collect())
}

