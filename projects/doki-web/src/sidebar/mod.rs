use super::*;

mod data;
mod item;

pub fn SideNav(cx: Scope) -> Element {
    let a = SideNavGroupItem::Simple { 0: SideNavItemSimple::default() };
    let b = SideNavGroupItem::List { 0: SideNavItemList::default() };

    cx.render(rsx! {
        nav {
            class: "bg-white flex flex-col xl:w-64",
            div {
                SideNavGroup {
                    title: "Title".to_string(),
                    items: vec![
                         a,
                         b
                    ],
                }
            }
        }
    })
}

pub struct SideNav {
    page: String,
}

#[derive(Debug, Clone, Props, PartialEq)]
pub struct SideNavGroupData {
    #[props(optional)]
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

#[derive(Debug, Clone, Props, PartialEq)]
pub struct SideNavItemSimple {
    name: String,
    #[props(optional)]
    icon: Option<SideNavItemIcon>,
    link: String,
}

#[derive(Debug, Clone, Props, PartialEq)]
pub struct SideNavItemList {
    button: String,
    #[props(optional)]
    can_close: Option<bool>,
    #[props(optional)]
    start_close: Option<bool>,
    list: Vec<SideNavItemList>,
}

impl SideNavGroupData {
    pub fn render_title(&self) -> Option<LazyNodes> {
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

pub fn SideNavGroup(cx: Scope<SideNavGroupData>) -> Element {
    let title = cx.props.render_title();
    let items = cx.props.items.iter().map(|e| e.render());
    cx.render(rsx! {
        ul {
            title
            items
        }
    })
}
