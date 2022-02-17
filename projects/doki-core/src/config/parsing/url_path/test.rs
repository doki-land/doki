use doki_error::Url;
use crate::{
    config::sidebar::{SidebarGroup, SidebarItem, SidebarList},
    DokiConfig, DokiLanguages, DokiSidebar, DokiVersion,
};

#[test]
fn test2() {
    let config = DokiConfig { url_base: vec!["config".to_string()], url_end: "html".to_string(), ..Default::default() };
    let lang = DokiLanguages { mode: Default::default(), ..Default::default() };
    let ver = DokiVersion { mode: Default::default(), ..Default::default() };
    let sidebar = DokiSidebar { section: "section".to_string(), ..Default::default() };
    let group = SidebarGroup { rewrite_url: None, ..Default::default() };
    let list = SidebarList { rewrite_url: Some(vec!["list".to_string()]), ..Default::default() };
    let item = SidebarItem { name: "name".to_string(), ..Default::default() };
    // config level
    let url = config.get_link().unwrap();
    assert_eq!(url.to_string(), "https://localhost/config/");
    // language level
    let url = lang.get_link(&url, "en-US").unwrap();
    assert_eq!(url.to_string(), "https://localhost/config/?language=en-US");
    // version level
    let url = ver.get_link(&url, "v1.0.0-alpha").unwrap();
    assert_eq!(url.to_string(), "https://localhost/config/?language=en-US&version=v1.0.0-alpha");
    // section level
    let url = sidebar.get_link(&url).unwrap();
    assert_eq!(url.to_string(), "https://localhost/config/section");
    // group level
    let url = group.get_link(&url).unwrap();
    assert_eq!(url.to_string(), "https://localhost/config/section");
    // list level
    let url = list.get_link(&url).unwrap();
    assert_eq!(url.to_string(), "https://localhost/config/list/");
    // item level
    let url = item.get_link_relative(&url, ".php").unwrap();
    assert_eq!(url, "config/list/.php")
}
