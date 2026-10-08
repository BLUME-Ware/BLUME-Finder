//! Removing a folder while another one is being indexed: each side uses its own connection, as
//! the application does, and neither may fail with "database is locked".

use blume_finder_core::{Index, Progress, Report};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
};

fn folder_with(dir: &Path, name: &str, texts: &[&str]) -> PathBuf {
    let folder = dir.join(name);
    fs::create_dir_all(&folder).unwrap();
    for (i, text) in texts.iter().enumerate() {
        fs::write(folder.join(format!("{i}.txt")), text).unwrap();
    }
    folder.canonicalize().unwrap()
}

/// Indexes `kept` in a background thread, pauses it when `pause_at` files have been seen, and
/// removes the already indexed `removed` folder during that pause.
fn remove_during_indexing(
    name: &str,
    pause_at: u64,
) -> (Result<u64, String>, Result<Report, String>, Vec<String>) {
    let dir = std::env::temp_dir().join(format!("blume-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let db = dir.join("index.db");
    let removed = folder_with(&dir, "removed", &["wombat"]);
    let kept = folder_with(&dir, "kept", &["quokka", "platypus"]);
    let mut index = Index::open(&db).unwrap();
    index.index_folder(&removed, |_| {}).unwrap();

    let (paused, wait_for_pause) = mpsc::channel();
    let (resume, wait_for_resume) = mpsc::channel();
    let indexer = thread::spawn({
        let db = db.clone();
        move || {
            let mut index = Index::open(&db).unwrap();
            index
                .index_folder(&kept, |progress: &Progress| {
                    if progress.seen == pause_at {
                        paused.send(()).unwrap();
                        wait_for_resume.recv().unwrap();
                    }
                })
                .map_err(|e| e.to_string())
        }
    });

    wait_for_pause.recv().unwrap();
    let removal = Index::open(&db)
        .unwrap()
        .forget_root(&removed.to_string_lossy())
        .map_err(|e| e.to_string());
    resume.send(()).unwrap();
    let indexing = indexer.join().unwrap();

    let roots = Index::open(&db).unwrap().roots().unwrap();
    let _ = fs::remove_dir_all(&dir);
    (
        removal,
        indexing,
        roots.into_iter().map(|(root, _)| root).collect(),
    )
}

#[test]
fn a_folder_can_be_removed_while_indexing_holds_written_files() {
    let (removal, indexing, roots) = remove_during_indexing("locked-write", 2);
    assert!(removal.is_ok(), "removal failed: {:?}", removal.err());
    assert!(indexing.is_ok(), "indexing failed: {:?}", indexing.err());
    assert_eq!(roots.len(), 1);
    assert!(roots[0].ends_with("kept"));
}

#[test]
fn indexing_survives_a_folder_removed_before_its_first_write() {
    let (removal, indexing, roots) = remove_during_indexing("stale-read", 1);
    assert!(removal.is_ok(), "removal failed: {:?}", removal.err());
    assert!(indexing.is_ok(), "indexing failed: {:?}", indexing.err());
    assert_eq!(roots.len(), 1);
    assert!(roots[0].ends_with("kept"));
}
