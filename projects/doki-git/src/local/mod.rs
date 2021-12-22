use chrono::NaiveDateTime;
use git2::{Blame, BlameHunk, Oid, Repository};

use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use std::path::{Path, PathBuf};
use jwalk::{DirEntry, WalkDirGeneric};
use project_root::get_project_root;

mod commit;
mod walk_dir;

pub use self::commit::{FileCommit, FileCommitItem};
pub use self::walk_dir::group_file;

#[test]
fn open() {
    let root = get_project_root().unwrap();
    let repo = Repository::open(root).unwrap();
    let file = repo.blame_file(&Path::new("projects/doki-git/src/lib.rs"), None).unwrap();
    let record = FileCommit::from(file);
    println!("{:#?}", record)
}

#[derive(Default, Debug)]
pub struct FileState {}

pub type FileResult<T> = jwalk::Result<DirEntry<T>>;

#[test]
fn walk() {
    let root = get_project_root().unwrap();
    let mut skip = HashSet::new();
    skip.insert("md".to_string());
    skip.insert("html".to_string());
    println!("{:#?}", group_file(root, Some(&skip)))
}
