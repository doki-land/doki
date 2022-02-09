use super::*;

pub struct DokiSidebar {
    pub enable: bool,
    /// section name of page sidebar
    pub section: String,
    pub items: Vec<SidebarGroup>
}

impl Default for DokiSidebar {
    fn default() -> Self {
        Self {
            enable: true,
            section: String::new(),
            items: vec![]
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SidebarGroup {
    title: Option<String>,
    items: Vec<SideNavGroupItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SideNavGroupItem {
    Simple(SideNavItemSimple),
    List(SideNavItemList),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SideNavItemIcon {
    Numeric(Vec<usize>),
    Icon(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SideNavItemSimple {
    name: String,
    icon: Option<SideNavItemIcon>,
    link: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SideNavItemList {
    button: String,
    can_close: Option<bool>,
    start_close: Option<bool>,
    list: Vec<SideNavItemList>,
}

impl Default for DokiVersion {
    fn default() -> Self {
        Self { enable: false, mode: Default::default(), head: vec![String::from("latest")] }
    }
}

impl DokiVersion {
    pub fn parse(raw: Value) -> Self {
        let default = Self::default();
        let root = match raw.into_table() {
            Ok(o) => o,
            Err(_) => return default,
        };
        let enable = parse_bool(&root, "enable").unwrap_or(default.enable);
        let head = parse_string_list(&root, "head").unwrap_or(default.head);
        let mode = DokiUrlMode::parse(&root, "mode").unwrap_or_default();
        Self { enable, mode, head }
    }
}

#[test]
fn test_sidebar() {
    let cfg = load_config_string(include_str!("version.json5"), FileFormat::Json5);
    println!("{:#?}", DokiVersion::parse(cfg.cache));
}