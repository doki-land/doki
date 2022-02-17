use super::*;

#[test]
fn test_sidebar() {
    let cfg = load_config_string(include_str!("sidebar.json5"), FileFormat::Json5);
    let parsed = DokiSidebar::parse(cfg.cache);
    println!("{}", parsed.dump_debug().unwrap());
}
