use super::*;



#[component(DocumentHeader < G >)]
pub fn builder() -> View<G> {
    header().build()
}