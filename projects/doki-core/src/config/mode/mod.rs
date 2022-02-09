use std::str::FromStr;
use super::*;


pub enum DokiUrlMode {
    HtmlData,
    UrlPath,
    UrlParameter,
    SubDomain,
}

impl DokiUrlMode {
    pub fn parse() {

    }
}

impl FromStr for  DokiUrlMode{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}