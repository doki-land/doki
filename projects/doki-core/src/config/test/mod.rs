use super::*;

#[test]
fn config() {
    let out = json5::from_str::<DokiConfig>(include_str!("config.json5"));
    debug_assert_eq!(format!("{:#?}", out), include_str!("config.yaml"),
    );
}
