#![allow(non_snake_case)]

mod article;
mod navbar;
mod sidebar;
mod utils;
mod widgets;

pub use self::{
    article::Article,
    navbar::Headnav,
    sidebar::SideNav,
    utils::*,
    widgets::{BackToTop, FloatNav},
};
use dioxus::prelude::*;
use rsass::compile_scss;
use doki_core::DokiSidebar;
use crate::sidebar::SideNavData;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(AppWeb)
}

pub trait DioxusRender {
    fn build_node(&self) -> LazyNodes;
}

pub fn main_ssr() {
    let mut vdom = VirtualDom::new(AppWeb);
    let _ = vdom.rebuild();
    println!("{}", dioxus::ssr::render_vdom(&vdom));
}

pub fn AppWeb(cx: Scope) -> Element {

    let side_nav = DokiSidebar::default();
    cx.render(rsx! {
        Headnav {

        }
        main {
            class: "flex flex-row",
            SideNav {
                data: side_nav,
            }
            Article {

            }
            BackToTop {

            }
            FloatNav {

            }
        }

    })
}
