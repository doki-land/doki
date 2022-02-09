use super::*;

#[track_caller]
pub fn scss(input: &str) -> String {
    match compile_scss(input.as_bytes(), Default::default()) {
        Ok(o) => String::from_utf8_lossy(&o).to_string(),
        Err(e) => {
            panic!("{}", e)
        }
    }
}
