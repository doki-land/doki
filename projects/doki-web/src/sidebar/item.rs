use super::*;

impl SideNavGroupItem {
    #[inline]
    pub fn render(&self) -> LazyNodes {
        match self {
            Self::Simple(s) => s.render(),
            Self::List(s) => s.render(),
        }
    }
}

impl SideNavItemIcon {
    pub fn render(&self) -> LazyNodes {
        match self {
            Self::Numeric(_) => {
                rsx! {
                    span {
                        "1."
                    }
                }
            }
            Self::Icon(_) => {
                rsx! {
                    svg {
                    class: "w-1 h-1",
                }
                }
            }
        }
    }
}

impl SideNavItemSimple {
    pub fn render(&self) -> LazyNodes {
        let icon = self.icon.as_ref().map(|icon| icon.render());
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

impl SideNavItemList {
    pub fn render(&self) -> LazyNodes {
        let text = self.button.as_str();
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
