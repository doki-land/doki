use sycamore::component::Component;
use sycamore::prelude::*;
use doki_core::pages::ContentPage;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    let night = Signal::new(false);

    sycamore::render(|| ContentPage::create_component(night.handle()));
}
