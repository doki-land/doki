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
    items: Vec<String>
}

pub enum SideNavGroupItem {

}


impl Default for SideNavGroupData {
    fn default() -> Self {
        Self {
            title: None,
            items: vec![]
        }
    }
}

impl SideNavGroupData {
    pub fn render_title<'a, 'b>(&'a self, cx: &'b Scope<'a, Self>) -> Option<LazyNodes<'a,'b>> {
        cx.props.title.as_ref().map(|title| {
            rsx! {
                li {
                    class: "flex flex-row",
                    "{title}"
                }
            }
        })
    }
}

pub fn SideNavGroup(cx: Scope<SideNavGroupData>) -> Element {
    let title = cx.props.render_title(&cx);
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
