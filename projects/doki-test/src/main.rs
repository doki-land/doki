use doki_core::pages::ContentPage;
use sycamore::{component::Component, prelude::*};

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    let night = Signal::new(false);

    sycamore::render(|| ContentPage::create_component(night.handle()));
}
