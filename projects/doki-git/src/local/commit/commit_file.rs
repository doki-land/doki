pub struct FileCommit {
    // TODO: use BTreeSet
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

impl From<Blame<'_>> for FileCommit  {
    fn from(file: Blame) -> Self {
        let mut record = FileCommit {
            inner: Default::default()
        };
        for i in file.iter() {
            record.insert(FileCommitItem::from(i));
        }
        return record
    }
}

impl Debug for FileCommit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.inner.values()).finish()
    }
}