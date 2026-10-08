//! Hidden files and well-known secret files stay out of the index, by name and by content.

use blume_finder_core::Index;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn folder_with(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("blume-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("docs")).unwrap();
    for (file, text) in files {
        fs::write(dir.join("docs").join(file), text).unwrap();
    }
    dir
}

fn indexed(dir: &Path) -> Index {
    let mut index = Index::open(&dir.join("index.db")).unwrap();
    index.index_folder(&dir.join("docs"), |_| {}).unwrap();
    index
}

fn found(index: &Index, query: &str) -> bool {
    !index.search(query, 10).unwrap().is_empty()
}

#[test]
fn hidden_files_are_not_indexed() {
    let dir = folder_with(
        "hidden",
        &[
            (".netrc", "machine example.com login alice password hunter2"),
            (".notes.md", "the zanzibar plan"),
            ("report.md", "the kangaroo report"),
        ],
    );
    let index = indexed(&dir);

    assert!(found(&index, "kangaroo"));
    assert!(!found(&index, "netrc"), ".netrc is indexed by name");
    assert!(
        !found(&index, "zanzibar"),
        ".notes.md is indexed by content"
    );
    assert!(dir.join("docs/.netrc").exists());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn credential_and_password_exports_are_not_indexed() {
    let dir = folder_with(
        "exports",
        &[
            ("credentials.csv", "User name,Password\nalice,quokka"),
            (
                "alice_accessKeys.csv",
                "Access key ID,Secret access key\nAKIA,wombat",
            ),
            (
                "Chrome Passwords.csv",
                "name,url,username,password\nbank,,alice,platypus",
            ),
            ("github-recovery-codes.txt", "echidna-1234\nechidna-5678"),
            ("budget.csv", "month,amount\nmarch,kangaroo"),
        ],
    );
    let index = indexed(&dir);

    assert!(found(&index, "kangaroo"));
    for secret in ["quokka", "wombat", "platypus", "echidna", "credentials"] {
        assert!(!found(&index, secret), "{secret} is indexed");
    }
    assert!(dir.join("docs/credentials.csv").exists());
    let _ = fs::remove_dir_all(&dir);
}
