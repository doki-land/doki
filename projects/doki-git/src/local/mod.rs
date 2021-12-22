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

    repo.reflog("a");
    for i in file.iter() {
        //i.final_signature()
        println!("{:?}", i.orig_commit_id());
        println!("{:?}", i.lines_in_hunk());
        let sig = i.orig_signature();
        println!("{:?}", sig.name());
        println!("{:?}", sig.email());
        println!("{:?}", NaiveDateTime::from_timestamp(sig.when().seconds(), 0));
        println!();
    }
}

#[derive(Debug)]
pub struct FileCommitItem {
    id: Oid,
    lines: usize,
    name: Option<String>,
    email: Option<String>,
    time: NaiveDateTime,
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