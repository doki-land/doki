use crate::DokiConfig;

#[test]
fn test2() {
    let config = DokiConfig {
        url_base: vec!["a".to_string()],
        url_end: "html".to_string(),
        version: Default::default(),
        i18n: Default::default()
    };
    println!("{:?}", config.get_link())
}