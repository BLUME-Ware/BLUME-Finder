//! Text removed from the index is erased from the files on disk, not only marked as deleted:
//! neither the extracted text nor its stems may remain in the database, its journal or its
//! shared memory file while the index is still open, as it is in the application.

use blume_finder_core::{text::Lang, Index};
use std::{
    fs,
    path::{Path, PathBuf},
};

const WORD: &str = "quixotrombone";

fn setup(name: &str) -> (PathBuf, PathBuf) {
    let dir = std::env::temp_dir().join(format!("blume-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let folder = dir.join("folder");
    fs::create_dir_all(&folder).unwrap();
    fs::write(folder.join("note.txt"), format!("the {WORD} agreement")).unwrap();
    (dir, folder.canonicalize().unwrap())
}

/// Whether `needle` appears in the index file or in any file SQLite keeps next to it.
fn on_disk(dir: &Path, needle: &str) -> bool {
    fs::read_dir(dir).unwrap().any(|entry| {
        let entry = entry.unwrap();
        entry.file_name().to_string_lossy().starts_with("index.db")
            && fs::read(entry.path())
                .unwrap()
                .windows(needle.len())
                .any(|w| w == needle.as_bytes())
    })
}

fn assert_erased(dir: &Path) {
    let stem = Lang::new().stem(WORD);
    assert!(!on_disk(dir, WORD), "the removed text is still on disk");
    assert!(!on_disk(dir, &stem), "the removed stem is still on disk");
}

#[test]
fn removing_a_folder_erases_its_text() {
    let (dir, folder) = setup("erase-folder");
    let mut index = Index::open(&dir.join("index.db")).unwrap();
    index.index_folder(&folder, |_| {}).unwrap();
    assert!(on_disk(&dir, WORD), "the text was never written");

    index.forget_root(&folder.to_string_lossy()).unwrap();
    assert_erased(&dir);
    drop(index);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn changing_a_file_erases_its_previous_text() {
    let (dir, folder) = setup("erase-changed");
    let mut index = Index::open(&dir.join("index.db")).unwrap();
    index.index_folder(&folder, |_| {}).unwrap();
    assert!(on_disk(&dir, WORD), "the text was never written");

    fs::write(
        folder.join("note.txt"),
        "a completely different and longer text",
    )
    .unwrap();
    index.index_folder(&folder, |_| {}).unwrap();
    assert_erased(&dir);
    drop(index);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn deleting_a_file_erases_its_text() {
    let (dir, folder) = setup("erase-deleted");
    let mut index = Index::open(&dir.join("index.db")).unwrap();
    index.index_folder(&folder, |_| {}).unwrap();
    assert!(on_disk(&dir, WORD), "the text was never written");

    fs::remove_file(folder.join("note.txt")).unwrap();
    index.index_folder(&folder, |_| {}).unwrap();
    assert_erased(&dir);
    drop(index);
    let _ = fs::remove_dir_all(&dir);
}
