use super::*;

impl DioxusRender for SidebarGroupItemKind {
    fn build_node(&self) -> LazyNodes {
        match self {
            Self::Simple(s) => s.build_node(),
            Self::List(s) => s.build_node(),
        }
    }
}

impl DioxusRender for SidebarItem {
    fn build_node(&self) -> LazyNodes {
        let icon = self.icon.as_ref().map(|icon| icon.build_node());
        let text = self.name.as_str();
        let link = self.link.as_str();
        rsx! {
            li {
                class: "flex flex-row",
                icon
                a {
                    href: "{link}",
                    "{text}"
                }
            }
        }
    }
}

impl DioxusRender for SidebarItemIcon {
    fn build_node(&self) -> LazyNodes {
        match self {
            Self::Numeric(_) => {
                rsx! {
                    span {
                        "1."
                    }
                }
            }
            Self::IconSVG(_) => {
                rsx! {
                    svg {
                        class: "w-1 h-1",
                    }
                }
            }
            Self::IconLink(_) => {
                rsx! {
                    img {
                        class: "w-1 h-1",
                    }
                }
            }
        }
    }
}

impl DioxusRender for SidebarList {
    fn build_node(&self) -> LazyNodes {
        let text = self.title.as_str();
        rsx! {
            button {
                "{text}"
            }
            li {
                class: "flex flex-col",
                button {
                    class: "flex flex-row",
                    svg {
                        class: "w-1 h-1",
                    }
                    "click"
                    svg {
                        class: "w-1 h-1",
                    }
                }
                ul {
                    "TExt"
                }
            }
        }
    }
}
