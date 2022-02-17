mod item;

use super::*;
use doki_core::{
    sidebar::{SidebarGroup, SidebarGroupItemKind, SidebarItem, SidebarItemIcon, SidebarList},
    DokiSidebar,
};

#[derive(Props, PartialEq)]
pub struct SideNavData {
    data: DokiSidebar,
}

pub fn SideNav(cx: Scope<SideNavData>) -> Element {
    cx.render(cx.props.data.build_node())
}

impl DioxusRender for DokiSidebar {
    fn build_node(&self) -> LazyNodes {
        rsx! {
            nav {
                class: "bg-white flex flex-col xl:w-64",
                div {
                    "side"
                }
            }
        }
    }
}

impl DioxusRender for SidebarGroup {
    fn build_node(&self) -> LazyNodes {
        let title = self.title.as_ref().map(|title| {
            rsx! {
                li {
                    class: "flex flex-row",
                    "{title}"
                }
            }
        });
        let items = self.items.iter().map(|e| e.build_node());

        rsx! {
            ul {
                title
                items
            }
        }
    }
}
