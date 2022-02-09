use doki_git::local::DirectoryWalker;
use project_root::get_project_root;
use std::collections::HashSet;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn open() {
    let root = get_project_root().unwrap();
    let mut select = HashSet::new();
    select.insert("md".to_string());
    select.insert("html".to_string());

    let mut walker = DirectoryWalker::new(root.as_path()).unwrap();
    walker.set_selected_extensions(select);

    println!("{:#?}", walker.group_repo(&root))
}
