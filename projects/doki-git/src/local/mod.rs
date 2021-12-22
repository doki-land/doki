use std::fmt::{Debug, Formatter};
use chrono::NaiveDateTime;
use git2::{Blame, BlameHunk, Oid, Repository};
use std::collections::BTreeMap;
use std::path::Path;
use project_root::get_project_root;

mod commit;

pub use self::commit::{FileCommit, FileCommitItem};


#[test]
fn open() {
    let root = get_project_root().unwrap();
    let repo = Repository::open(root).unwrap();
    let file = repo.blame_file(&Path::new("Cargo.toml"), None).unwrap();
    let record = FileCommit::from(file);
    println!("{:#?}", record)
}




