use super::*;

pub fn SideNav(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class: "bg-white flex flex-col xl:w-64",
            div {
                // SideNavGroup {
                //     title: "Title".to_string(),
                //     items: vec![],
                // }
            }
        }
    })
}

#[derive(Props, PartialEq)]
pub struct SideNavGroupData {
    #[props(optional)]
    title: Option<String>,
    items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SideNavGroupItemIcon {
    Numeric,
    Icon(String),
}

#[derive(Props, PartialEq)]
pub struct SideNavGroupItemSimple {
    #[props(optional)]
    icon: Option<SideNavGroupItemIcon>,
    name: String,
    link: String,
}

#[derive(Props, PartialEq)]
pub struct SideNavGroupItemList {
    button: String,
    #[props(optional)]
    can_close: Option<bool>,
    #[props(optional)]
    start_close: Option<bool>,
    list: Vec<SideNavGroupItemList>,
}

pub enum SideNavGroupItem {
    Simple(SideNavGroupItemSimple),
    List(SideNavGroupItemList),
}

impl Default for SideNavGroupData {
    fn default() -> Self {
        Self { title: None, items: vec![] }
    }
}

impl SideNavGroupData {
    pub fn render_title<'a>(&'a self) -> Option<LazyNodes<'a, '_>> {
        self.title.as_ref().map(|title| {
            rsx! {
                li {
                    class: "flex flex-row",
                    "{title}"
                }
            }
        })
    }
}

impl SideNavGroupItemSimple {
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

impl SideNavGroupItemIcon {
    pub fn render(&self) -> LazyNodes {
        match self {
            Self::Numeric => {
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

pub fn SideNavGroup(cx: Scope<SideNavGroupData>) -> Element {
    let title = cx.props.render_title();
    cx.render(rsx! {
        ul {
                title
                li {
                    class: "flex flex-row",
                    svg {
                        class: "w-1 h-1",
                    }
                    a {
                        "Text"
                    }
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
    })
}
