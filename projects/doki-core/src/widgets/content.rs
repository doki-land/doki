mod footer;

use super::*;

pub use self::footer::{DocumentFooter, DocumentNav};

#[component(DocumentRoot < G >)]
fn builder(is_night: ReadSignal<bool>) -> View<G> {
    body()
        .dyn_class("night", is_night)
        // .dyn_class("light", !is_night)
        .child(DocumentHeader::create_component(()))
        .child(DocumentSidebar::create_component(()))
        .child(DocumentContext::create_component(()))
        .build()
}

#[component(DocumentHeader < G >)]
fn builder() -> View<G> {
    header().build()
}

#[component(DocumentSidebar < G >)]
fn builder() -> View<G> {
    aside().build()
}

pub struct AdjacentPages {
    pub prev: Option<(String, String)>,
    pub next: Option<(String, String)>,
}

fn new() {
    let _nav = AdjacentPages { prev: Some(("a".to_string(), "b".to_string())), next: Some(("c".to_string(), "d".to_string())) };
}

#[component(DocumentContext < G >)]
fn builder() -> View<G> {
    let nav = AdjacentPages { prev: Some(("a".to_string(), "b".to_string())), next: Some(("c".to_string(), "d".to_string())) };
    let nav = Signal::new(nav);

    main()
        .child(DocumentArticle::create_component(()))
        .child(DocumentFooter::create_component(()))
        .child(DocumentNav::create_component(nav.handle()))
        .build()
}

#[component(DocumentArticle < G >)]
fn builder() -> View<G> {
    article().build()
}
