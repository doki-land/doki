#[cfg(feature = "non-wasm")]
mod parser;

mod path_builder;
mod url_builder;

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DokiUrlMode {
    HtmlData,
    UrlPath,
    UrlQuery { short: bool },
    SubDomain,
}

#[derive(Clone, PartialEq, Hash)]
pub struct UrlBuilder {
    domain: Vec<String>,
    host: String,
    base: Vec<String>,
    path: Vec<String>,
    query: BTreeMap<String, String>,
    end: String,
}

impl Default for DokiUrlMode {
    fn default() -> Self {
        Self::UrlQuery { short: false }
    }
}

/// Encodes a u64 hash to a lowercase base-36 string.
///
/// For case-insensitive usage.
pub fn base36(value: u64) -> String {
    const R: i32 = 36;
    const M: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut v = value.into();
    let mut digits: Vec<u8> = vec![];
    while v > 0 {
        let digit = (v % R) as usize;
        v /= R;
        digits.push(M[digit]);
    }

    digits.reverse();
    unsafe { format!("{:0>8}", String::from_utf8_unchecked(digits)) }
}
