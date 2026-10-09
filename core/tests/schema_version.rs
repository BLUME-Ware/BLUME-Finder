//! The index records the version of its format. An index written before the version existed is
//! brought to version 1, and an index from a newer version is refused without being modified.

use blume_finder_core::Index;
use rusqlite::Connection;
use std::{fs, path::PathBuf};

fn fresh_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("blume-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn user_version(db: &std::path::Path) -> i64 {
    Connection::open(db)
        .unwrap()
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap()
}

#[test]
fn a_new_index_is_at_version_1() {
    let dir = fresh_dir("schema-new");
    let db = dir.join("index.db");
    drop(Index::open(&db).unwrap());
    assert_eq!(user_version(&db), 1);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn an_index_without_version_gets_english_statuses() {
    let dir = fresh_dir("schema-v0");
    let db = dir.join("index.db");
    {
        let conn = Connection::open(&db).unwrap();
        conn.execute_batch(
            "CREATE TABLE files(
                 id INTEGER PRIMARY KEY,
                 path TEXT NOT NULL UNIQUE,
                 name TEXT NOT NULL,
                 ext TEXT NOT NULL,
                 size INTEGER NOT NULL,
                 mtime INTEGER NOT NULL,
                 status TEXT NOT NULL,
                 root TEXT NOT NULL
             );
             CREATE INDEX files_root ON files(root);
             CREATE TABLE texts(file_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
             CREATE VIRTUAL TABLE fts USING fts5(
                 name, stems, tokenize='unicode61 remove_diacritics 2'
             );
             INSERT INTO files VALUES
                 (1, '/r/a.txt', 'a.txt', 'txt', 1, 1, 'ok', '/r'),
                 (2, '/r/b.png', 'b.png', 'png', 1, 1, 'nom_seul', '/r'),
                 (3, '/r/c.pdf', 'c.pdf', 'pdf', 1, 1, 'illisible', '/r'),
                 (4, '/r/d.txt', 'd.txt', 'txt', 1, 1, 'vide', '/r'),
                 (5, '/r/e.pdf', 'e.pdf', 'pdf', 1, 1, 'trop_gros', '/r');
             INSERT INTO texts VALUES (1, 'hello');",
        )
        .unwrap();
    }

    let index = Index::open(&db).unwrap();
    let stats = index.stats().unwrap();
    assert_eq!(
        (
            stats.files,
            stats.with_text,
            stats.name_only,
            stats.unreadable
        ),
        (5, 1, 1, 3)
    );
    drop(index);

    let conn = Connection::open(&db).unwrap();
    let mut st = conn
        .prepare("SELECT status FROM files ORDER BY id")
        .unwrap();
    let statuses: Vec<String> = st
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(
        statuses,
        ["ok", "name_only", "unreadable", "empty", "too_large"]
    );
    assert_eq!(user_version(&db), 1);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn an_index_from_a_newer_version_is_refused_untouched() {
    let dir = fresh_dir("schema-future");
    let db = dir.join("index.db");
    {
        let conn = Connection::open(&db).unwrap();
        conn.execute_batch("CREATE TABLE future(x); PRAGMA user_version=2;")
            .unwrap();
    }
    let before = fs::read(&db).unwrap();

    assert!(Index::open(&db).is_err());
    assert_eq!(
        fs::read(&db).unwrap(),
        before,
        "the newer index was modified"
    );
    assert_eq!(user_version(&db), 2);
    let _ = fs::remove_dir_all(&dir);
}
