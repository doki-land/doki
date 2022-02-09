use super::*;

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
