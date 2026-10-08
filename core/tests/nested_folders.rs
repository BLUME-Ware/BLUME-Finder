//! Indexed folders never overlap: a subfolder of an indexed folder is already covered, and an
//! indexed subfolder becomes part of a parent folder indexed later.

use blume_finder_core::Index;
use std::{fs, path::PathBuf};

/// `docs/a.txt` and `docs/work/w.txt`, with the canonical paths of `docs` and `docs/work`.
fn docs_with_work(name: &str) -> (PathBuf, PathBuf, PathBuf) {
    let dir = std::env::temp_dir().join(format!("blume-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("docs/work")).unwrap();
    fs::write(dir.join("docs/a.txt"), "the kangaroo report").unwrap();
    fs::write(dir.join("docs/work/w.txt"), "the wombat contract").unwrap();
    let docs = dir.join("docs").canonicalize().unwrap();
    let work = docs.join("work");
    (dir, docs, work)
}

fn found(index: &Index, query: &str) -> bool {
    !index.search(query, 10).unwrap().is_empty()
}

#[test]
fn a_subfolder_of_an_indexed_folder_is_not_indexed_again() {
    let (dir, docs, work) = docs_with_work("nested-child");
    let mut index = Index::open(&dir.join("index.db")).unwrap();
    index.index_folder(&docs, |_| {}).unwrap();

    assert!(index.index_folder(&work, |_| {}).is_err());
    assert_eq!(
        index.roots().unwrap(),
        vec![(docs.to_string_lossy().to_string(), 2)]
    );
    index.forget_root(&work.to_string_lossy()).unwrap();
    assert!(
        found(&index, "wombat"),
        "removing the subfolder hid its files"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_parent_folder_takes_over_its_indexed_subfolder() {
    let (dir, docs, work) = docs_with_work("nested-parent");
    let mut index = Index::open(&dir.join("index.db")).unwrap();
    index.index_folder(&work, |_| {}).unwrap();

    let report = index.index_folder(&docs, |_| {}).unwrap();
    assert_eq!(report.unchanged, 1, "the subfolder's file was read again");
    assert_eq!(
        index.roots().unwrap(),
        vec![(docs.to_string_lossy().to_string(), 2)]
    );
    assert!(found(&index, "wombat"));
    let _ = fs::remove_dir_all(&dir);
}
