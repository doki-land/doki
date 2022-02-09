use super::*;
use config::Map;

impl SidebarGroupItemKind {
    pub fn item<S>(name: S, link: S, icon: Option<SidebarItemIcon>) -> Self
    where
        S: Into<String>,
    {
        SidebarGroupItemKind::Simple(SidebarItem { icon, name: name.into(), link: link.into() })
    }
    pub fn list<S>(title: S, list: Vec<SidebarList>, closed: (bool, bool)) -> Self
    where
        S: Into<String>,
    {
        SidebarGroupItemKind::List(SidebarList {
            title: "".to_string(),
            path: "".to_string(),
            url: "".to_string(),
            icon: None,
            foldable: false,
            folded: false,
            items: vec![],
        })
    }
}

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
                if let Ok(item) = i.into_table() {
                    match item.get("name") {
                        None => continue,
                        Some(_) => out.push(SidebarItem::parse(item)),
                    }
                    match item.get("list") {
                        None => continue,
                        Some(_) => out.push(SidebarList::parse(item)),
                    }
                }
            }
        }
        out
    }
}

impl SidebarGroupItemKind {
    pub fn parse(root: Value) -> Option<Self> {
        let root = root.into_table().ok()?;
        match item.get("name") {
            None => {}
            Some(_) => out.push(SidebarItem::parse(item)),
        }
        match item.get("list") {
            None => {}
            Some(_) => out.push(SidebarList::parse(item)),
        }
        None
    }
}

impl SidebarItem {
    pub fn parse(root: Map<String, Value>) -> SidebarGroupItemKind {
        let root = root.into_table()?;
        let title = parse_string(&root, "group");
        Self { title, items: Self::parse_items(&root) }
    }
}
