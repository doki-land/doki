use crate::{
    config::sidebar::{SidebarGroup, SidebarItem, SidebarList},
    DokiConfig, DokiLanguages, DokiSidebar, DokiVersion,
};

#[test]
fn test2() {
    let config = DokiConfig { url_base: vec!["config".to_string()], url_end: "php".to_string(), ..Default::default() };
    let lang = DokiLanguages { mode: Default::default(), ..Default::default() };
    let ver = DokiVersion { mode: Default::default(), ..Default::default() };
    let sidebar = DokiSidebar { section: "section".to_string(), ..Default::default() };
    let group = SidebarGroup { rewrite_url: None, ..Default::default() };
    let list = SidebarList { rewrite_url: Some(vec!["list".to_string()]), ..Default::default() };
    let item = SidebarItem { name: "name".to_string(), ..Default::default() };
    // config level
    let url = config.get_link();
    assert_eq!(url, "/config");
    // language level
    let url = lang.get_link(&url, "en-US");
    assert_eq!(url, "/config?language=en-US");
    // version level
    let url = ver.get_link(&url, "v1.0.0-alpha");
    assert_eq!(url, "/config?language=en-US&version=v1.0.0-alpha");
    // section level
    let url = sidebar.get_link(&url);
    assert_eq!(url, "/config/section?language=en-US&version=v1.0.0-alpha");
    // group level
    let url = group.get_link(&url);
    assert_eq!(url, "/config/section?language=en-US&version=v1.0.0-alpha");
    // list level
    let url = list.get_link(&url);
    assert_eq!(url, "/config/section/list?language=en-US&version=v1.0.0-alpha");
    // item level
    let mut url = item.get_link(&url);
    assert_eq!(url, "/config/section/list/name?language=en-US&version=v1.0.0-alpha");
    // end level
    config.set_link_end(&mut url);
    assert_eq!(url, "/config/section/list/name.php?language=en-US&version=v1.0.0-alpha");
}
