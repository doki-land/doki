use super::*;

pub fn Article(cx: Scope) -> Element {
    cx.render(rsx! {
        section {
            class: "bg-gray-600",
            header {
                class: "",
            }
            article {
                class: ""
            }
            footer {
                class: ""
            }
        }
    })
}

pub fn Link(cx: Scope) -> Element {
    cx.render(rsx! {
        link {
        }
    })
}
