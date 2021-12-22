mod commit;
mod walk_dir;

pub use self::{
    commit::{FileCommit, FileCommitItem},
    walk_dir::DirectoryWalker,
};

use chrono::NaiveDateTime;
use git2::{Blame, BlameHunk, Oid, Repository};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap, HashSet},
    fmt::{Debug, Formatter},
};
use jwalk::{DirEntry, WalkDirGeneric};
use project_root::get_project_root;
use std::path::{Path, PathBuf};
use std::ops::AddAssign;
use url::Url;
use crate::Result;

impl DirectoryWalker {
    pub fn group_repo<P: AsRef<Path>>(&self, root: P) -> Result<BTreeMap<String, FileCommit>> {
        let mut out = Default::default();
        let repo = Repository::open(root)?;
        for (e, files) in self.group_file(&root) {

        }


        let file = repo.blame_file(&Path::new("projects/doki-git/src/lib.rs"), None)?;
        let record = FileCommit::new(file);
        return Ok(out);
    }
}


#[test]
fn open() {
    let root = get_project_root().unwrap();
    let repo = Repository::open(root).unwrap();
    let file = repo.blame_file(&Path::new("projects/doki-git/src/lib.rs"), None).unwrap();
    let record = FileCommit::from(file);
    println!("{:#?}", record)
}

#[test]
fn walk() {
    let root = get_project_root().unwrap();
    let mut skip = HashSet::new();
    skip.insert("md".to_string());
    skip.insert("html".to_string());

    let walker = DirectoryWalker {
        extension_select: skip
    };

    println!("{:#?}", walker.group_file(root))
}
