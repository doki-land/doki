use super::*;

pub type FileResult<T> = jwalk::Result<DirEntry<T>>;

pub struct DirectoryWalker {
    repository: Repository,
    extension_select: HashSet<String>,
}

impl Debug for DirectoryWalker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("DirectoryWalker");
        w.field("repository", &self.repository.path().to_string_lossy());
        if !self.extension_select.is_empty() {
            w.field("selected", &self.extension_select);
        }
        w.finish()
    }
}

impl DirectoryWalker {
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self> {
        Ok(Self { repository: Repository::open(root)?, extension_select: Default::default() })
    }
    pub fn set_selected_extensions(&mut self, extensions: HashSet<String>) {
        self.extension_select = extensions;
    }
}

impl DirectoryWalker {
    pub fn group_repo<P: AsRef<Path>>(&self, directory: P) -> Result<BTreeMap<Url, FileCommit>> {
        let mut out = BTreeMap::default();
        for (_, files) in self.group_file(directory.as_ref()) {
            for path in files {
                match FileCommit::new(&self.repository, &path) {
                    Ok(o) => {
                        out.insert(o.get_url(), o);
                    }
                    Err(_) => continue,
                }
            }
        }
        return Ok(out);
    }
    pub fn group_file<P: AsRef<Path>>(&self, root: P) -> BTreeMap<String, Vec<PathBuf>> {
        let walk_dir = WalkDirGeneric::<(usize, bool)>::new(root).process_read_dir(|_, _, _, children| {
            // children.sort_by(file_sort);
            children.iter_mut().for_each(ignore_dot);
        });
        let mut grouped = BTreeMap::default();
        for entry in walk_dir {
            let file = match entry {
                Ok(maybe_file) if maybe_file.file_type.is_file() => maybe_file,
                _ => continue,
            };
            let path = file.path();
            let ext = match path.extension().and_then(|f| f.to_str()) {
                Some(s) if self.extension_select.is_empty() => s,
                Some(s) if self.extension_select.contains(s) => s,
                _ => continue,
            };
            match grouped.get_mut(ext) {
                None => {
                    grouped.insert(ext.to_string(), vec![path]);
                }
                Some(s) => {
                    s.push(path);
                }
            }
        }
        return grouped;
    }
}

#[inline]
#[allow(unused)]
fn file_sort(a: &FileResult<(usize, bool)>, b: &FileResult<(usize, bool)>) -> Ordering {
    match (a, b) {
        (Ok(a), Ok(b)) => a.file_name.cmp(&b.file_name),
        (Ok(_), Err(_)) => Ordering::Less,
        (Err(_), Ok(_)) => Ordering::Greater,
        (Err(_), Err(_)) => Ordering::Equal,
    }
}

#[inline]
#[allow(unused)]
fn ignore_dot(result: &mut FileResult<(usize, bool)>) {
    let dir = match result {
        Ok(maybe_dir) if maybe_dir.file_type.is_dir() => maybe_dir,
        _ => return,
    };
    match dir.file_name().to_str() {
        Some(s) if s.starts_with('.') => dir.read_children_path = None,
        // Some(s) if ignores.contains(s) => dir.read_children_path = None,
        _ => (),
    }
}
