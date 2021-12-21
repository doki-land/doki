use sycamore::prelude::*;
use sycamore::builder::html::*;


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

#[component(DocumentContext < G >)]
fn builder() -> View<G> {
    main()
        .child(DocumentArticle::create_component(()))
        .child(DocumentFooter::create_component(()))
        .child(DocumentNav::create_component(()))
        .build()
}


#[component(DocumentArticle < G >)]
fn builder() -> View<G> {
    article().build()
}


#[component(DocumentFooter < G >)]
fn builder() -> View<G> {
    article().build()
}

#[component(DocumentNav < G >)]
fn builder() -> View<G> {
    nav().build()
}


