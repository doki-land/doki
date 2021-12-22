use std::collections::HashSet;
use project_root::get_project_root;
use doki_git::local::DirectoryWalker;

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
