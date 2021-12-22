use std::fmt::Pointer;
use super::*;

impl Debug for FileCommit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("FileCommit");
        w.field("path", &self.url.path());
        w.field("commits",&self.inner.values() );
        w.finish()
    }
}

impl AddAssign<Self> for FileCommit {
    fn add_assign(&mut self, rhs: Self) {
        for (time, commit) in rhs.inner {
            match self.inner.get_mut(&time) {
                None => {
                    self.inner.insert(time, commit);
                }
                Some(s) => {
                    s.lines += commit.lines
                }
            }
        }
    }
}

impl FileCommit {
    pub fn insert(&mut self, item: FileCommitItem) {
        let key = item.time.to_owned();
        match self.inner.get_mut(&key) {
            None => {
                self.inner.insert(key, item);
            }
            Some(s) => s.lines += item.lines,
        }
    }
    pub fn new(repo: &Repository, path: &Path) -> Result<Self> {
        let file = repo.blame_file(Self::relative_path(path, repo.path())?.as_path(), None)?;
        let mut record = FileCommit { url: Url::from_file_path(&path)?, inner: Default::default() };
        for i in file.iter() {
            record.insert(FileCommitItem::from(i));
        }
        return Ok(record);
    }

    fn relative_path(path: &Path, base: &Path)-> Result<PathBuf> {
        let s = match diff_paths(path, base).map(|s| s.to_path_buf()) {
            None => {return Err(todo!())}
            Some(s) => {s.to_string_lossy().trim_start_matches("..\\").replace("\\", "/")}
        };
        Ok(PathBuf::from(s))
    }
}