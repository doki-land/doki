use super::*;

impl Default for DokiSidebar {
    fn default() -> Self {
        Self { enable: true, section: String::new(), url: None, items: vec![] }
    }
}

impl Default for SidebarGroup {
    fn default() -> Self {
        Self { title: None, items: vec![] }
    }
}

impl Default for SidebarGroupItemKind {
    fn default() -> Self {
        Self::Simple(Default::default())
    }
}

impl Default for SidebarItem {
    fn default() -> Self {
        Self { icon: None, name: "Test Item".to_string(), link: String::new(), url: None }
    }
}

impl Default for SidebarList {
    fn default() -> Self {
        Self {
            title: "Test List".to_string(),
            path: "".to_string(),
            url: "".to_string(),
            icon: None,
            foldable: false,
            folded: false,
            items: vec![],
        }
    }
}
