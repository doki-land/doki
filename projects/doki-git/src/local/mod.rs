mod commit;
mod walk_dir;

pub use self::{
    commit::{FileCommit, FileCommitItem},
    walk_dir::DirectoryWalker,
};
use pathdiff::diff_paths;
use chrono::NaiveDateTime;
use git2::{BlameHunk, Oid, Repository};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
    fmt::{Debug, Formatter},
};
use jwalk::{DirEntry, WalkDirGeneric};
use project_root::get_project_root;
use std::path::{Path, PathBuf};
use std::ops::AddAssign;
use url::Url;
use crate::Result;

impl DirectoryWalker {
    pub fn group_repo<P: AsRef<Path>>(&self, root: P) -> Result<BTreeMap<String, Vec<FileCommit>>> {
        let mut out = BTreeMap::default();
        let repo = Repository::open(root.as_ref())?;
        for (e, files) in self.group_file(root.as_ref()) {
            let mut commits = Vec::with_capacity(files.capacity());
            for path in files {
                match FileCommit::new(&repo, &path) {
                    Ok(o) => { commits.push(o)}
                    Err(_) => {continue}
                }
            }
            out.insert(e, commits);
        }
        return Ok(out);
    }
}


#[test]
fn open() {
    let root = get_project_root().unwrap();
    let mut skip = HashSet::new();
    skip.insert("md".to_string());
    skip.insert("html".to_string());

    let walker = DirectoryWalker {
        extension_select: skip
    };
    println!("{:#?}", walker.group_repo(root))
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
