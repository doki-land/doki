use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};
use chrono::NaiveDateTime;
use git2::{BlameHunk, Oid, Repository};
use project_root::get_project_root;


#[test]
fn open() {
    let root = get_project_root().unwrap();
    ;
    let repo = Repository::open(root).unwrap();
    let file = repo.blame_file(&Path::new("Cargo.toml"), None).unwrap();
    let mut record = FileCommit {
        inner: Default::default()
    };
    for i in file.iter() {
        record.insert(FileCommitItem::from(i));
    }
    println!("{:#?}", record)
}

pub struct FileCommit {
    inner: BTreeMap<NaiveDateTime, FileCommitItem>,
}


impl FileCommit {



    pub fn insert(&mut self, item: FileCommitItem) {
        let key = item.time.to_owned();
        match self.inner.get_mut(&key) {
            None => { self.inner.insert(key, item); }
            Some(s) => {
                s.lines += item.lines
            }
        }
    }
}

impl Debug for FileCommit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.inner.values()).finish()
    }
}


pub struct FileCommitItem {
    id: Oid,
    lines: usize,
    name: Option<String>,
    email: Option<String>,
    time: NaiveDateTime,
}


impl Debug for FileCommitItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("Commit");
        w.field("time", &self.time);
        w.field("id", &self.id);
        w.field("lines", &self.lines);
        if let Some(s) = &self.name { w.field("name", s); }
        if let Some(s) = &self.name { w.field("email", s); }
        w.finish()
    }
}


impl From<BlameHunk<'_>> for FileCommitItem {
    fn from(blame: BlameHunk) -> Self {
        let sig = blame.orig_signature();
        Self {
            id: blame.orig_commit_id(),
            lines: blame.lines_in_hunk(),
            name: sig.name().map(|s| s.to_string()),
            email: sig.email().map(|s| s.to_string()),
            time: NaiveDateTime::from_timestamp(sig.when().seconds(), 0),
        }
    }
}