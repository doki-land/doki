use super::*;

mod commit_file;
mod commit_item;

pub struct FileCommit {
    // TODO: use BTreeSet
    inner: BTreeMap<NaiveDateTime, FileCommitItem>,
}

pub struct FileCommitItem {
    id: Oid,
    lines: usize,
    name: Option<String>,
    email: Option<String>,
    time: NaiveDateTime,
}
