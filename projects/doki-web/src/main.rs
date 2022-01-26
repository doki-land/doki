#![allow(non_snake_case)]

mod navbar;
mod sidebar;
mod article;
mod widgets;

use dioxus::prelude::*;
pub use self::navbar::Headnav;
pub use self::sidebar::SideNav;
pub use self::article::Article;
pub use self::widgets::{BackToTop, FloatNav};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(AppWeb)
}

pub fn main_ssr() {
    let mut vdom = VirtualDom::new(AppWeb);
    let _ = vdom.rebuild();
    println!("{}", dioxus::ssr::render_vdom(&vdom));
}

pub fn AppWeb(cx: Scope) -> Element {
    cx.render(rsx! {
        Headnav {

        }
        main {
            class: "flex flex-row",
            SideNav {

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
