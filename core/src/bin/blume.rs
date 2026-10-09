//! Command-line interface of Blume Finder (to test the engine).
//!
//!   blume index <folder> [--db file]
//!   blume search <query> [--db file] [--limit N] [--json]
//!   blume stats [--db file]

use blume_finder_core::{text, Index};
use std::{
    env,
    path::{Path, PathBuf},
    process,
};

fn usage() -> ! {
    eprintln!(
        "Usage:\n  blume index <folder> [--db file]\n  blume search <query> [--db file] [--limit N] [--json]\n  blume stats [--db file]"
    );
    process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut db = PathBuf::from("blume-index.db");
    let mut limit = 10usize;
    let mut json = false;
    let mut rest: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--db" => {
                i += 1;
                db = PathBuf::from(args.get(i).unwrap_or_else(|| usage()));
            }
            "--limit" => {
                i += 1;
                limit = args
                    .get(i)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or_else(|| usage());
            }
            "--json" => json = true,
            other => rest.push(other.to_string()),
        }
        i += 1;
    }
    if rest.is_empty() {
        usage();
    }

    if let Err(e) = run(&rest, &db, limit, json) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(rest: &[String], db: &Path, limit: usize, json: bool) -> blume_finder_core::Result<()> {
    let mut index = Index::open(db)?;
    match rest[0].as_str() {
        "index" => {
            let folder = rest.get(1).unwrap_or_else(|| usage());
            // Some damaged PDFs make the library panic: the file is skipped silently.
            let previous = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let report = index.index_folder(&PathBuf::from(folder), |_| {});
            std::panic::set_hook(previous);
            let report = report?;
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!(
                    "{} files seen, {} indexed, {} unchanged, {} removed, {} findable by name only, {} secrets skipped, {} errors ({} ms)",
                    report.seen, report.indexed, report.unchanged, report.removed,
                    report.name_only, report.skipped_sensitive, report.errors, report.millis
                );
            }
        }
        "search" => {
            let query = rest[1..].join(" ");
            let hits = index.search(&query, limit)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&hits)?);
            } else if hits.is_empty() {
                println!("No results for \"{query}\".");
            } else {
                for (n, h) in hits.iter().enumerate() {
                    println!("{}. {}  ({} KB)", n + 1, h.name, h.size / 1024);
                    println!("   {}", h.path);
                    if let Some(s) = &h.snippet {
                        let s = s
                            .replace(text::HL_START, "\x1b[1m")
                            .replace(text::HL_END, "\x1b[0m");
                        println!("   {s}");
                    }
                }
            }
        }
        "folders" => {
            let roots = index.roots()?;
            if json {
                let v: Vec<_> = roots
                    .iter()
                    .map(|(p, n)| serde_json::json!({"path": p, "files": n}))
                    .collect();
                println!("{}", serde_json::to_string(&v)?);
            } else {
                for (p, n) in roots {
                    println!("{n} files  {p}");
                }
            }
        }
        "forget" => {
            let folder = rest.get(1).unwrap_or_else(|| usage());
            let n = index.forget_root(folder)?;
            println!("{n} files removed from the index (the files themselves are not touched).");
        }
        "stats" => {
            let s = index.stats()?;
            if json {
                println!("{}", serde_json::to_string_pretty(&s)?);
            } else {
                println!(
                    "{} files, of which {} read in full, {} findable by name only, {} unreadable ({} KB of text)",
                    s.files, s.with_text, s.name_only, s.unreadable, s.text_bytes / 1024
                );
            }
        }
        _ => usage(),
    }
    Ok(())
}
