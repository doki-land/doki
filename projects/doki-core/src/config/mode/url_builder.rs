use std::fmt::{Display, Formatter};
use super::*;


impl Display for UrlBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('/')?;


        if !self.query.is_empty() {

        }

        if !self.end.is_empty() {
            write!(f, ".{}", self.end)?
        }
        Ok(())
    }
}

impl UrlBuilder {
    pub fn query_insert(&mut self, key: String, value: String) -> Option<String> {
        self.query.insert(key, value)
    }
    pub fn query_clear(&mut self) {
        self.query.clear()
    }
}