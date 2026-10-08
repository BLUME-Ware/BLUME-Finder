//! Blume Finder : moteur de recherche dans le contenu de tes fichiers.
//!
//! Règles de ce moteur :
//! - il ne fait aucune requête réseau (aucune dépendance réseau) ;
//! - il ne modifie, ne déplace et ne supprime jamais un fichier : il lit seulement ;
//! - il n'indexe jamais les fichiers de secrets (clés, .env, trousseaux) ni les
//!   dossiers cachés, applications et dossiers de dépendances ;
//! - l'index contient le texte de tes fichiers : il reste sur ta machine.

pub mod extract;
pub mod text;

use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    time::{Instant, UNIX_EPOCH},
};
use walkdir::{DirEntry, WalkDir};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Serialize)]
pub struct Hit {
    pub path: String,
    pub name: String,
    pub ext: String,
    pub size: i64,
    pub mtime: i64,
    /// Extrait avec les mots trouvés entre `text::HL_START` et `text::HL_END`.
    /// Absent quand le fichier n'a été trouvé que par son nom.
    pub snippet: Option<String>,
    pub score: f64,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Report {
    pub seen: u64,
    pub indexed: u64,
    pub unchanged: u64,
    pub removed: u64,
    /// Fichiers trouvables seulement par leur nom (images, scans, formats non lus).
    pub name_only: u64,
    pub skipped_sensitive: u64,
    pub errors: u64,
    pub millis: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Progress {
    pub seen: u64,
    pub indexed: u64,
    pub current: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Stats {
    pub files: i64,
    pub with_text: i64,
    pub name_only: i64,
    pub unreadable: i64,
    pub text_bytes: i64,
}

pub struct Index {
    conn: Connection,
    lang: text::Lang,
}

impl Index {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        // L'index contient le texte de tes fichiers : lecture réservée à toi.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o600));
        }
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             CREATE TABLE IF NOT EXISTS files(
                 id INTEGER PRIMARY KEY,
                 path TEXT NOT NULL UNIQUE,
                 name TEXT NOT NULL,
                 ext TEXT NOT NULL,
                 size INTEGER NOT NULL,
                 mtime INTEGER NOT NULL,
                 status TEXT NOT NULL,
                 root TEXT NOT NULL
             );
             CREATE INDEX IF NOT EXISTS files_root ON files(root);
             CREATE TABLE IF NOT EXISTS texts(file_id INTEGER PRIMARY KEY, body TEXT NOT NULL);
             CREATE VIRTUAL TABLE IF NOT EXISTS fts USING fts5(
                 name, stems, tokenize='unicode61 remove_diacritics 2'
             );",
        )?;
        Ok(Index {
            conn,
            lang: text::Lang::new(),
        })
    }

    /// Indexe un dossier. Les fichiers inchangés (même taille, même date) sont ignorés,
    /// les fichiers disparus sont retirés de l'index.
    pub fn index_folder(
        &mut self,
        root: &Path,
        on_progress: impl FnMut(&Progress),
    ) -> Result<Report> {
        self.conn.execute_batch("BEGIN")?;
        match self.index_inner(root, on_progress) {
            Ok(report) => {
                self.conn.execute_batch("COMMIT")?;
                Ok(report)
            }
            Err(e) => {
                let _ = self.conn.execute_batch("ROLLBACK");
                Err(e)
            }
        }
    }

    fn index_inner(
        &mut self,
        root: &Path,
        mut on_progress: impl FnMut(&Progress),
    ) -> Result<Report> {
        let t0 = Instant::now();
        let root = root.canonicalize()?;
        let root_s = root.to_string_lossy().to_string();

        let mut known: HashMap<String, (i64, i64, i64)> = HashMap::new();
        {
            let mut st = self
                .conn
                .prepare("SELECT id, path, size, mtime FROM files WHERE root=?1")?;
            let rows = st.query_map([&root_s], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, i64>(2)?,
                    r.get::<_, i64>(3)?,
                ))
            })?;
            for row in rows {
                let (id, p, size, mtime) = row?;
                known.insert(p, (id, size, mtime));
            }
        }

        let home = home_dir();
        let mut seen: HashSet<String> = HashSet::new();
        let mut rep = Report::default();
        let mut batch = 0u32;

        let walker = WalkDir::new(&root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| e.depth() == 0 || !is_excluded(e, home.as_deref()));

        for entry in walker {
            let Ok(entry) = entry else {
                rep.errors += 1;
                continue;
            };
            if !entry.file_type().is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if is_sensitive_name(&name) {
                rep.skipped_sensitive += 1;
                continue;
            }
            let Ok(meta) = entry.metadata() else {
                rep.errors += 1;
                continue;
            };
            let size = meta.len() as i64;
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);
            let path_s = entry.path().to_string_lossy().to_string();

            rep.seen += 1;
            seen.insert(path_s.clone());
            if let Some(&(_, sz, mt)) = known.get(&path_s) {
                if sz == size && mt == mtime {
                    rep.unchanged += 1;
                    continue;
                }
            }

            on_progress(&Progress {
                seen: rep.seen,
                indexed: rep.indexed,
                current: path_s.clone(),
            });

            let ext = Path::new(&name)
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            let (status, body) = if extract::is_readable(&ext) {
                if size as u64 > extract::MAX_FILE_BYTES {
                    ("trop_gros", None)
                } else {
                    match extract::extract_text(entry.path(), &ext) {
                        Some(t) if !t.trim().is_empty() => ("ok", Some(t)),
                        Some(_) => ("vide", None),
                        None => ("illisible", None),
                    }
                }
            } else {
                ("nom_seul", None)
            };
            if body.is_none() {
                rep.name_only += 1;
            }

            self.store(
                &path_s,
                &name,
                &ext,
                size,
                mtime,
                status,
                &root_s,
                body.as_deref(),
            )?;
            rep.indexed += 1;
            batch += 1;
            if batch >= 200 {
                self.conn.execute_batch("COMMIT; BEGIN")?;
                batch = 0;
            }
        }

        for (path, (id, _, _)) in &known {
            if !seen.contains(path) {
                self.conn.execute("DELETE FROM fts WHERE rowid=?1", [id])?;
                self.conn
                    .execute("DELETE FROM texts WHERE file_id=?1", [id])?;
                self.conn.execute("DELETE FROM files WHERE id=?1", [id])?;
                rep.removed += 1;
            }
        }

        rep.millis = t0.elapsed().as_millis() as u64;
        Ok(rep)
    }

    #[allow(clippy::too_many_arguments)]
    fn store(
        &self,
        path: &str,
        name: &str,
        ext: &str,
        size: i64,
        mtime: i64,
        status: &str,
        root: &str,
        body: Option<&str>,
    ) -> Result<()> {
        let id: i64 = self.conn.query_row(
            "INSERT INTO files(path, name, ext, size, mtime, status, root)
             VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(path) DO UPDATE SET
                 name=excluded.name, ext=excluded.ext, size=excluded.size,
                 mtime=excluded.mtime, status=excluded.status, root=excluded.root
             RETURNING id",
            params![path, name, ext, size, mtime, status, root],
            |r| r.get(0),
        )?;
        self.conn.execute("DELETE FROM fts WHERE rowid=?1", [id])?;
        self.conn
            .execute("DELETE FROM texts WHERE file_id=?1", [id])?;

        let name_words = text::name_words(name);
        let mut stems = text::stem_text(&self.lang, &name_words);
        if let Some(b) = body {
            stems.push(' ');
            stems.push_str(&text::stem_text(&self.lang, b));
            self.conn.execute(
                "INSERT INTO texts(file_id, body) VALUES(?1, ?2)",
                params![id, b],
            )?;
        }
        self.conn.execute(
            "INSERT INTO fts(rowid, name, stems) VALUES(?1, ?2, ?3)",
            params![id, name_words, stems],
        )?;
        Ok(())
    }

    /// Cherche d'abord les fichiers qui contiennent tous les mots importants de la
    /// requête ; s'il n'y en a aucun, ceux qui en contiennent au moins un.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<Hit>> {
        let q = text::parse_query(&self.lang, query);
        if q.terms.is_empty() {
            return Ok(vec![]);
        }
        let mut hits = self.run(&q, "AND", limit)?;
        if hits.is_empty() && q.terms.len() > 1 {
            hits = self.run(&q, "OR", limit)?;
        }
        Ok(hits)
    }

    fn run(&self, q: &text::Query, op: &str, limit: usize) -> Result<Vec<Hit>> {
        let expr = text::fts_expr(q, op);
        let mut st = self.conn.prepare(
            "SELECT rowid, bm25(fts, 8.0, 1.0) AS s FROM fts WHERE fts MATCH ?1 ORDER BY s LIMIT ?2",
        )?;
        let ranked: Vec<(i64, f64)> = st
            .query_map(params![expr, limit as i64], |r| Ok((r.get(0)?, r.get(1)?)))?
            .collect::<std::result::Result<_, _>>()?;

        let mut hits = Vec::with_capacity(ranked.len());
        for (id, score) in ranked {
            let (path, name, ext, size, mtime): (String, String, String, i64, i64) =
                self.conn.query_row(
                    "SELECT path, name, ext, size, mtime FROM files WHERE id=?1",
                    [id],
                    |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
                )?;
            let body: Option<String> = self
                .conn
                .query_row("SELECT body FROM texts WHERE file_id=?1", [id], |r| {
                    r.get(0)
                })
                .optional()?;
            let snippet = body.and_then(|b| text::make_snippet(&self.lang, &b, q));
            hits.push(Hit {
                path,
                name,
                ext,
                size,
                mtime,
                snippet,
                score: -score,
            });
        }
        Ok(hits)
    }

    /// Dossiers indexés, avec le nombre de fichiers de chacun.
    pub fn roots(&self) -> Result<Vec<(String, i64)>> {
        let mut st = self
            .conn
            .prepare("SELECT root, COUNT(*) FROM files GROUP BY root ORDER BY root")?;
        let rows = st
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Retire un dossier de l'index. Les fichiers eux-mêmes ne sont pas touchés.
    pub fn forget_root(&mut self, root: &str) -> Result<u64> {
        self.conn.execute_batch("BEGIN")?;
        let res: Result<u64> = (|| {
            self.conn.execute(
                "DELETE FROM fts WHERE rowid IN (SELECT id FROM files WHERE root=?1)",
                [root],
            )?;
            self.conn.execute(
                "DELETE FROM texts WHERE file_id IN (SELECT id FROM files WHERE root=?1)",
                [root],
            )?;
            Ok(self
                .conn
                .execute("DELETE FROM files WHERE root=?1", [root])? as u64)
        })();
        match res {
            Ok(n) => {
                self.conn.execute_batch("COMMIT")?;
                Ok(n)
            }
            Err(e) => {
                let _ = self.conn.execute_batch("ROLLBACK");
                Err(e)
            }
        }
    }

    /// Vrai si ce chemin figure dans l'index (sert à n'ouvrir que des fichiers connus).
    pub fn is_indexed(&self, path: &str) -> Result<bool> {
        Ok(self
            .conn
            .query_row("SELECT 1 FROM files WHERE path=?1", [path], |_| Ok(()))
            .optional()?
            .is_some())
    }

    pub fn stats(&self) -> Result<Stats> {
        let count = |sql: &str| -> Result<i64> { Ok(self.conn.query_row(sql, [], |r| r.get(0))?) };
        Ok(Stats {
            files: count("SELECT COUNT(*) FROM files")?,
            with_text: count("SELECT COUNT(*) FROM files WHERE status='ok'")?,
            name_only: count("SELECT COUNT(*) FROM files WHERE status='nom_seul'")?,
            unreadable: count(
                "SELECT COUNT(*) FROM files WHERE status IN ('illisible','vide','trop_gros')",
            )?,
            text_bytes: count("SELECT COALESCE(SUM(LENGTH(body)),0) FROM texts")?,
        })
    }
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

/// Dossiers jamais parcourus : cachés, applications, dépendances, caches système.
fn is_excluded(e: &DirEntry, home: Option<&Path>) -> bool {
    if !e.file_type().is_dir() {
        return false;
    }
    let name = e.file_name().to_string_lossy().to_lowercase();
    if name.starts_with('.') {
        return true;
    }
    if matches!(
        name.as_str(),
        "node_modules"
            | "__pycache__"
            | "venv"
            | "site-packages"
            | "$recycle.bin"
            | "system volume information"
    ) {
        return true;
    }
    if name.ends_with(".app") || name.ends_with(".framework") || name.ends_with(".photoslibrary") {
        return true;
    }
    // Dossiers système de l'utilisateur (Library sur Mac, AppData sur Windows).
    if matches!(name.as_str(), "library" | "appdata") && e.path().parent() == home {
        return true;
    }
    false
}

/// Fichiers de secrets : jamais lus, jamais indexés, même par leur nom.
fn is_sensitive_name(name: &str) -> bool {
    let n = name.to_lowercase();
    n.starts_with(".env")
        || n == "id_rsa"
        || n == "id_ed25519"
        || n == "id_ecdsa"
        || n == ".htpasswd"
        || n == "credentials"
        || n.ends_with(".pem")
        || n.ends_with(".key")
        || n.ends_with(".p12")
        || n.ends_with(".pfx")
        || n.ends_with(".kdbx")
        || n.ends_with(".keychain")
        || n.ends_with(".keychain-db")
        || n.ends_with(".gpg")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_can_move_between_threads() {
        fn is_send<T: Send>() {}
        is_send::<Index>();
    }

    #[test]
    fn roots_and_forget() {
        let dir = std::env::temp_dir().join(format!("blume-test-{}", std::process::id()));
        let docs = dir.join("docs");
        fs::create_dir_all(&docs).unwrap();
        fs::write(docs.join("a.txt"), "Le loyer du mois de mars").unwrap();
        let mut idx = Index::open(&dir.join("t.db")).unwrap();
        let rep = idx.index_folder(&docs, |_| {}).unwrap();
        assert_eq!(rep.indexed, 1);
        let roots = idx.roots().unwrap();
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].1, 1);
        assert_eq!(idx.search("loyers", 5).unwrap().len(), 1);
        let p = idx.search("loyer", 5).unwrap()[0].path.clone();
        assert!(idx.is_indexed(&p).unwrap());
        assert!(!idx.is_indexed("/nope").unwrap());
        idx.forget_root(&roots[0].0).unwrap();
        assert!(idx.roots().unwrap().is_empty());
        assert!(idx.search("loyer", 5).unwrap().is_empty());
        assert!(
            docs.join("a.txt").exists(),
            "les fichiers ne doivent jamais être touchés"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn sensitive_names() {
        assert!(is_sensitive_name(".env.local"));
        assert!(is_sensitive_name("server.PEM"));
        assert!(!is_sensitive_name("environnement.txt"));
    }
}
