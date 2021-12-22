use super::*;

mod commit_file;
mod commit_item;

#[derive(Clone)]
pub struct FileCommit {
    url: Url,
    // TODO: use BTreeSet
    inner: BTreeMap<NaiveDateTime, FileCommitItem>,
}

#[derive(Clone)]
pub struct FileCommitItem {
    id: Oid,
    lines: usize,
    name: Option<String>,
    email: Option<String>,
    time: NaiveDateTime,
}
