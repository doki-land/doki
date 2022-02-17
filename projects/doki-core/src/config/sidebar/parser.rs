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
        let url = parse_string(&root, "url");
        Self { enable, section, url, items: Self::parse_items(&root) }
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
        let url = parse_url_fragment(&root, "url");
        Self { name, icon: None, url }
    }
}

impl SidebarList {
    pub fn parse(root: Map<String, Value>) -> Self {
        let default = Self::default();
        let title = parse_string(&root, "list").unwrap_or(default.title);
        let rewrite_path = parse_string(&root, "rewrite_path").or_else(|| parse_string(&root, "path"));
        let rewrite_url = parse_url_fragment(&root, "rewrite_url").or_else(|| parse_url_fragment(&root, "url"));
        // let icon
        let foldable = parse_bool(&root, "foldable").unwrap_or(default.foldable);
        let folded = parse_bool(&root, "folded").unwrap_or(default.folded);
        Self { title, rewrite_path, rewrite_url, icon: None, foldable, folded, items: vec![] }
    }
}
