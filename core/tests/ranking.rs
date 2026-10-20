//! A document whose text matches the query comes before files found by their name only, however
//! many of them there are.

use blume_finder_core::Index;
use std::fs;

#[test]
fn content_matches_come_before_name_only_matches() {
    let dir = std::env::temp_dir().join(format!("blume-ranking-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let docs = dir.join("docs");
    fs::create_dir_all(&docs).unwrap();
    fs::write(
        docs.join("releve.txt"),
        "Facture d'électricité du mois de mars",
    )
    .unwrap();
    for n in 0..40 {
        fs::write(docs.join(format!("Facture Loop {n}.ogg")), [0u8; 16]).unwrap();
    }
    let mut index = Index::open(&dir.join("index.db")).unwrap();
    index.index_folder(&docs, |_| {}).unwrap();

    let hits = index.search("facture", 30).unwrap();
    assert_eq!(hits.len(), 30);
    assert_eq!(
        hits[0].name, "releve.txt",
        "the document was buried under name-only files"
    );
    let _ = fs::remove_dir_all(&dir);
}
