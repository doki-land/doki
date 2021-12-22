mod footer;
mod header;

use super::*;

pub use self::footer::{DocumentFooter};
pub use self::header::DocumentHeader;

#[component(ContentPage < G >)]
pub fn builder(is_night: ReadSignal<bool>) -> View<G> {
    body()
        .dyn_class("night", is_night)
        // .dyn_class("light", !is_night)
        .child(ContentPageHeader::create_component(()))
        .child(DocumentFloatTOC::create_component(()))
        .child(ContentPageSidebar::create_component(()))
        .child(DocumentContext::create_component(()))
        .build()
}


#[component(ContentPageHeader < G >)]
pub fn builder() -> View<G> {
    nav()
        .class("nav-float-header")
        .build()
}

#[component(DocumentFloatTOC < G >)]
pub fn builder() -> View<G> {
    nav()
        .class("nav-float-sidebar")
        .build()
}


#[component(ContentPageSidebar < G >)]
fn builder() -> View<G> {
    aside().build()
}

pub struct AdjacentPages {
    pub prev: Option<(String, String)>,
    pub next: Option<(String, String)>,
}

#[component(DocumentContext < G >)]
fn builder() -> View<G> {


    main()
        .child(DocumentHeader::create_component(()))
        .child(DocumentArticle::create_component(()))
        .child(DocumentFooter::create_component(()))
        .child(DocumentFloatTOC::create_component(()))
        .build()
}

#[component(DocumentArticle < G >)]
fn builder() -> View<G> {
    article().build()
}
