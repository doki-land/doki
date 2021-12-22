use super::*;

#[component(DocumentFooter < G >)]
pub fn builder() -> View<G> {
    let nav = AdjacentPages { prev: Some(("s".to_string(), "上一页".to_string())), next: Some(("c".to_string(), "下一页".to_string())) };
    let nav = Signal::new(nav);
    footer()
        .child(DocumentFootJump::create_component(nav.handle()))
        .build()
}

// <div class="meta-item edit-time">
//     最后更新: 2019-10-24 12:32:00
//     <a class="edit-button" aria-label="icon: edit" target="_blank"
//        href="full link"
//
//     >
//         <svg></svg>
//     </a>
// </div>

#[component(DocumentFootJump < G >)]
pub fn builder(pages: ReadSignal<AdjacentPages>) -> View<G> {
    let prev = pages.get().prev.to_owned();
    let next = pages.get().next.to_owned();
    nav()
        .class("nav-content")
        .dyn_child(move || match &prev {
            None => View::empty(),
            Some(s) => nav_span("prev", &s.0, &s.1),
        })
        .dyn_child(move || match &next {
            None => View::empty(),
            Some(s) => nav_span("next", &s.0, &s.1),
        })
        .build()
}

#[inline]
pub fn nav_span<G: GenericNode>(kls: &str, link: &str, text: &str) -> View<G> {
    span().class(kls).child(a().class("nav-link").attr("aria-label", text).attr("href", link).text(text).build()).build()
}
