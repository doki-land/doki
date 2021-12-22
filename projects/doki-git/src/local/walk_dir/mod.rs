use super::*;

pub fn group_file<P: AsRef<Path>>(root: P, collects: Option<&HashSet<String>>)
                                  -> HashMap<String,Vec<PathBuf>> {
    let walk_dir = WalkDirGeneric::<(usize, bool)>::new(root)
        .process_read_dir(|_, _, _, children| {
            // children.sort_by(file_sort);
            children.iter_mut().for_each(dir_ignore);
        });
    let mut grouped = HashMap::new();
    for entry in walk_dir {
        let file = match entry {
            Ok(maybe_file) if maybe_file.file_type.is_file()=> {
                maybe_file
            }
            _ => {continue}
        };

        let path = file.path();
        let ext = match path.extension().and_then(|f| f.to_str()) {
            Some(s) => {
                match collects {
                    Some(c) if !c.contains(s) => {
                        continue
                    }
                    _ => {s},
                }

            },
            _ => {continue}
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

#[inline]
#[allow(unused)]
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
#[allow(unused)]
fn dir_ignore(result: &mut FileResult<(usize, bool)>) {
    let mut skip = HashSet::new();
    skip.insert("target");
    skip.insert("dist");
    let dir = match result {
        Ok(maybe_dir) if maybe_dir.file_type.is_dir() => {
            maybe_dir
        }
        _ => { return; }
    };
    match dir.file_name().to_str() {
        Some(s) if s.starts_with('.') => {
            dir.read_children_path = None
        }
        Some(s) if skip.contains(s) => {
            dir.read_children_path = None
        }
        _ => ()
    }
}