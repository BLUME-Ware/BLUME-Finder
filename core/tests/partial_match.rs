//! When no file contains every word of the query, the files containing some of them are still
//! returned, and the search says so.

use blume_finder_core::Index;
use std::fs;

#[test]
fn a_search_reports_when_not_every_word_was_found() {
    let dir = std::env::temp_dir().join(format!("blume-partial-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let docs = dir.join("docs");
    fs::create_dir_all(&docs).unwrap();
    fs::write(
        docs.join("releve.txt"),
        "Facture d'électricité, période du 01/03 au 31/03",
    )
    .unwrap();
    let mut index = Index::open(&dir.join("index.db")).unwrap();
    index.index_folder(&docs, |_| {}).unwrap();

    let all = index.find("facture électricité", 30).unwrap();
    assert!(all.every_word);
    assert_eq!(all.hits.len(), 1);

    let some = index.find("facture électricité mars", 30).unwrap();
    assert!(!some.every_word, "the missing word was not reported");
    assert_eq!(some.hits.len(), 1);
    assert_eq!(some.hits[0].name, "releve.txt");
    let _ = fs::remove_dir_all(&dir);
}
