use sycamore::prelude::*;
use sycamore::builder::html::{nav, header};

#[component(DocumentHeader<G>)]
fn doc_header(value: ReadSignal<i32>) -> View<G> {
    header().build()
}