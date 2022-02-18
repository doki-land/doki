use super::*;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use xxhash_rust::xxh3::{xxh3_64, Xxh3};

impl Default for UrlBuilder {
    fn default() -> Self {
        Self {
            domain: vec![],
            host: "localhost".to_string(),
            base: vec![],
            path: vec![],
            query: Default::default(),
            end: "".to_string(),
        }
    }
}

impl Debug for UrlBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://{domain}{host}", domain = self.domain.join("."), host = self.host)?;
        Display::fmt(self, f)
    }
}

impl Display for UrlBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let path: Vec<_> = self.base.iter().chain(self.path.iter()).collect();
        write!(f, "/{}", path.join("/"))?;
        if !self.end.is_empty() {
            write!(f, ".{}", self.end)?
        }
        if !self.query.is_empty() {
            let mut out = vec![];
            for (k, v) in &self.query {
                out.push(format!("{}={}", k, v))
            }
            write!(f, "?{}", out.join("&"))?;
        }
        Ok(())
    }
}

impl PartialEq<&str> for UrlBuilder {
    fn eq(&self, other: &&str) -> bool {
        self.to_string().eq(other)
    }
}

impl UrlBuilder {
    pub fn new() -> Self {
        let mut new = Self::default();
        new.base.extend_from_slice(path);
        new
    }
    pub fn domain_join<S>(&mut self, dom: S)
    where
        S: Into<String>,
    {
        self.domain.push(dom.into())
    }
    pub fn path_join<S>(&mut self, path: S)
    where
        S: Into<String>,
    {
        self.path.push(path.into())
    }
    pub fn path_extend(&mut self, path: &[String]) {
        self.path.extend_from_slice(path)
    }
    pub fn query_insert<S>(&mut self, key: S, value: S)
    where
        S: Into<String>,
    {
        self.query.insert(key.into(), value.into());
    }
    pub fn query_clear(&mut self) {
        self.query.clear()
    }
    pub fn set_extension<S>(&mut self, end: S)
    where
        S: Into<String>,
    {
        self.end = end.into()
    }
    /// base 36 url hash
    pub fn url_hashed(&self) -> String {
        let mut hasher = Xxh3::default();
        self.hash(&mut hasher);
        format!("/{}/{}", self.base.join("/"), base36(hasher.finish()))
    }
}
