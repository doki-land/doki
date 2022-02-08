use super::*;

impl Default for SideNavGroupData {
    fn default() -> Self {
        Self { title: None, items: vec![] }
    }
}

impl Default for SideNavItemSimple {
    fn default() -> Self {
        Self { icon: None, name: String::new(), link: String::new() }
    }
}

impl Default for SideNavItemList {
    fn default() -> Self {
        Self { button: String::new(), can_close: None, start_close: None, list: vec![] }
    }
}

impl SideNavGroupItem {
    pub fn item<S>(name: S, link: S, icon: Option<SideNavItemIcon>) -> Self
    where
        S: Into<String>,
    {
        SideNavGroupItem::Simple(SideNavItemSimple { icon, name: name.into(), link: link.into() })
    }
    pub fn list<S>(title: S, list: Vec<SideNavItemList>, closed: (bool, bool)) -> Self
    where
        S: Into<String>,
    {
        SideNavGroupItem::List(SideNavItemList {
            button: title.into(),
            can_close: Some(closed.0),
            start_close: Some(closed.1),
            list,
        })
    }
}
