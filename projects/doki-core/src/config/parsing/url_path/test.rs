use crate::{DokiConfig, DokiLanguages, DokiSidebar, DokiVersion};
use crate::config::sidebar::{SidebarGroup, SidebarItem, SidebarList};

#[test]
fn test2() {
    let config = DokiConfig {
        url_base: vec!["a".to_string()],
        url_end: "html".to_string(),
        ..Default::default()
    };
    let lang = DokiLanguages {
        mode: Default::default(),
        ..Default::default()
    };
    let ver = DokiVersion {
        mode: Default::default(),
        ..Default::default()
    };
    let sidebar = DokiSidebar {
        section: "section".to_string(),
        ..Default::default()
    };
    let group = SidebarGroup {
        rewrite_url: None,
        ..Default::default()
    };
    let list = SidebarList {
        rewrite_url: Some(vec!["list".to_string()]),
        ..Default::default()
    };
    let item = SidebarItem {
        name: "name".to_string(),
        ..Default::default()
    };
    let url = config.get_link().unwrap();
    let url = lang.get_link(&url, "en-US").unwrap();
    let url = ver.get_link(&url, "v1.0.0-alpha").unwrap();
    let url = sidebar.get_link(&url).unwrap();
    let url = group.get_link(&url).unwrap();
    let url = list.get_link(&url).unwrap();
    let url = item.get_link(&url).unwrap();
    println!("{:?}", url.to_string())
}