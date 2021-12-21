use super::*;

#[test]
fn config() {
    assert_eq!(
        json5::from_str(include_str!("config.json5")),
        Ok(DokiConfig { url_base: vec![String::from("a"), String::from("b")], url_end: Some(String::from(".html")), version: None, i18n: None }),
    );
}
