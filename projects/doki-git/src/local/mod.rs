use std::path::Path;
use git2::Repository;


#[test]
fn open() {
    let repo = match Repository::open("../../") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let file = repo.blame_file(&Path::new("Cargo.toml"), None).unwrap();
    for i in file.iter() {
        println!("{:#?}", i.orig_signature().when())
    }
}