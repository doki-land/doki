
use super::*;

impl Debug for FileCommit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.inner.values()).finish()
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
    pub fn new(url: Url, file: Blame) -> Self {
        let mut record = FileCommit { inner: Default::default() };
        for i in file.iter() {
            record.insert(FileCommitItem::from(i));
        }
        return record;
    }
}