use super::*;

pub struct DokiPath {
    version: String,
    language: String,
    path: Vec<String>,
}

impl DokiConfig {
    pub fn build_url(&self, path: &DokiPath) -> String {
        let mut out: String = self.url_base.join("/");
        if let true = self.version.enable {
            out.write_char('/').ok();
            out.write_str(&path.version).ok();
        }
        if let true = self.i18n.enable {
            out.write_char('/').ok();
            out.write_str(&path.language).ok();
        }
        for p in &path.path {
            out.write_char('/').ok();
            out.write_str(&p).ok();
        }
        out.push_str(&self.url_end);
        return out;
    }
}
