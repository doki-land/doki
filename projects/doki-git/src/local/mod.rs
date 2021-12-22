use chrono::NaiveDateTime;
use git2::{Blame, BlameHunk, Oid, Repository};

use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
};
use std::cmp::Ordering;
use std::ffi::OsString;
use std::path::Path;
use jwalk::{DirEntry, WalkDirGeneric};
use project_root::get_project_root;


mod commit;

pub use self::commit::{FileCommit, FileCommitItem};

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
    let walk_dir = WalkDirGeneric::<(usize, bool)>::new(root)
        .process_read_dir(|_, _, _, children| {
            children.sort_by(file_sort);
            children.retain(file_remove);
            children.iter_mut().for_each(file_skip);
        });

    for entry in walk_dir {
        println!("{}", entry.unwrap().path().display());
    }
}

#[inline]
fn file_sort(a: &FileResult<(usize, bool)>, b: &FileResult<(usize, bool)>) -> Ordering {
    match (a, b) {
        (Ok(a), Ok(b)) => {
            a.file_name.cmp(&b.file_name)
        }
        (Ok(_), Err(_)) => Ordering::Less,
        (Err(_), Ok(_)) => Ordering::Greater,
        (Err(_), Err(_)) => Ordering::Equal,
    }
}

#[inline]
fn file_remove(result: &FileResult<(usize, bool)>) -> bool {
    let file = match result {
        Ok(file) => { file }
        Err(_) => { return false; }
    };
    println!("{:?}", file.file_name);
    return true;
    // result.as_ref().map(|dir_entry| {
    //     dir_entry.file_name
    //         .to_str()
    //         .map(|s| s.starts_with('.'))
    //         .unwrap_or(true)
    // }).unwrap_or(true)
}

#[inline]
#[allow(unused)]
fn file_skip(result: &mut FileResult<(usize, bool)>) {
    if let Ok(maybe_dir) = result {
        if !maybe_dir.file_type.is_dir() {
            return;
        }
        // println!("{:?}", dir_entry.file_name);
        match maybe_dir.file_name().to_str() {
            Some("target") => { maybe_dir.read_children_path = None }
            Some(".git") => { maybe_dir.read_children_path = None }
            _ => ()
        }
    }
}