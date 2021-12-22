use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};
use chrono::NaiveDateTime;
use git2::{Blame, BlameHunk, Oid, Repository};
use project_root::get_project_root;
use url::Url;
use crate::Result;

mod commit;


#[test]
fn open() {
    let root = get_project_root().unwrap();
    let repo = Repository::open(root).unwrap();
    let file = repo.blame_file(&Path::new("Cargo.toml"), None).unwrap();
    let record = FileCommit::from(file);
    println!("{:#?}", record)
}




