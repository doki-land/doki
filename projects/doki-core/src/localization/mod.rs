use fluent_fallback::Localization;

#[test]
fn main() {
    // generate_messages is a closure that returns an iterator over FluentBundle
    // instances.
    let loc = Localization::new(vec!["simple.ftl".into()], false);

    let value = bundle.format_value("hello-world", None);

    assert_eq!(&value, "Hello, world!");
}
