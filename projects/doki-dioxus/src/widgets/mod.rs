use super::*;

pub fn BackToTop(cx: Scope) -> Element {
    let style = scss(include_str!("style.scss"));
    cx.render(rsx! {
        button {
            class: "back-to-top",
            title: "Back to Top",
            "Top"
        }
        style {"{style}"}
    })
}

pub fn FloatNav(cx: Scope) -> Element {
    cx.render(rsx! {
        aside {
            class: "bg-gray-800 float",
        }
    })
}
