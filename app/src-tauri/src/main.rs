// Blume Finder: Tauri layer.
// This layer only connects the interface to the local engine (`core` folder).
// It opens no network connection and does not modify, move or delete any file.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use blume_finder_core::{Found, Index, Report, Stats};
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::DialogExt;

#[derive(Serialize)]
struct Folder {
    path: String,
    files: i64,
}

fn db_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("blume-finder.db"))
}

fn open_index(app: &AppHandle) -> Result<Index, String> {
    Index::open(&db_path(app)?).map_err(|e| e.to_string())
}

/// Acts only on a file already in the index.
fn known_file(app: &AppHandle, path: &str) -> Result<(), String> {
    if open_index(app)?
        .is_indexed(path)
        .map_err(|e| e.to_string())?
    {
        Ok(())
    } else {
        Err("File not in the index.".into())
    }
}

#[tauri::command]
async fn choose_folder(app: AppHandle) -> Option<String> {
    app.dialog()
        .file()
        .blocking_pick_folder()
        .and_then(|p| p.into_path().ok())
        .map(|p| p.to_string_lossy().to_string())
}

#[derive(Serialize)]
struct KnownFolder {
    kind: &'static str,
    path: String,
}

/// The usual folders offered on first launch, only those that exist on this system. Their paths
/// are canonical, like the indexed folders, so the interface can show their translated names.
#[tauri::command]
fn default_folders(app: AppHandle) -> Vec<KnownFolder> {
    let paths = app.path();
    [
        ("documents", paths.document_dir()),
        ("desktop", paths.desktop_dir()),
        ("downloads", paths.download_dir()),
    ]
    .into_iter()
    .filter_map(|(kind, dir)| {
        let path = dir.ok()?.canonicalize().ok()?;
        path.is_dir().then(|| KnownFolder {
            kind,
            path: path.to_string_lossy().to_string(),
        })
    })
    .collect()
}

#[tauri::command]
async fn list_folders(app: AppHandle) -> Result<Vec<Folder>, String> {
    let roots = open_index(&app)?.roots().map_err(|e| e.to_string())?;
    Ok(roots
        .into_iter()
        .map(|(path, files)| Folder { path, files })
        .collect())
}

#[tauri::command]
async fn index_folder(app: AppHandle, path: String) -> Result<Report, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let mut index = open_index(&app)?;
        let mut last = Instant::now() - Duration::from_secs(1);
        let handle = app.clone();
        index
            .index_folder(Path::new(&path), move |p| {
                if last.elapsed() > Duration::from_millis(150) {
                    last = Instant::now();
                    let _ = handle.emit("progress", p);
                }
            })
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Removes a folder from the index. The files themselves are not touched.
#[tauri::command]
async fn forget_folder(app: AppHandle, path: String) -> Result<u64, String> {
    open_index(&app)?
        .forget_root(&path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn search(app: AppHandle, query: String) -> Result<Found, String> {
    open_index(&app)?
        .find(&query, 30)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn stats(app: AppHandle) -> Result<Stats, String> {
    open_index(&app)?.stats().map_err(|e| e.to_string())
}

/// Opens the file with the system's default application.
#[tauri::command]
async fn open_file(app: AppHandle, path: String) -> Result<(), String> {
    known_file(&app, &path)?;
    #[cfg(target_os = "macos")]
    let mut cmd = {
        let mut c = Command::new("open");
        c.arg(&path);
        c
    };
    #[cfg(target_os = "windows")]
    let mut cmd = {
        let mut c = Command::new("explorer");
        c.arg(&path);
        c
    };
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    let mut cmd = {
        let mut c = Command::new("xdg-open");
        c.arg(&path);
        c
    };
    cmd.spawn().map(|_| ()).map_err(|e| e.to_string())
}

/// Shows the file in the Finder (Mac) or the Explorer (Windows).
#[tauri::command]
async fn reveal_file(app: AppHandle, path: String) -> Result<(), String> {
    known_file(&app, &path)?;
    #[cfg(target_os = "macos")]
    let mut cmd = {
        let mut c = Command::new("open");
        c.arg("-R").arg(&path);
        c
    };
    #[cfg(target_os = "windows")]
    let mut cmd = {
        let mut c = Command::new("explorer");
        c.arg(format!("/select,{path}"));
        c
    };
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    let mut cmd = {
        let mut c = Command::new("xdg-open");
        c.arg(Path::new(&path).parent().unwrap_or(Path::new("/")));
        c
    };
    cmd.spawn().map(|_| ()).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            choose_folder,
            default_folders,
            list_folders,
            index_folder,
            forget_folder,
            search,
            stats,
            open_file,
            reveal_file
        ])
        .run(tauri::generate_context!())
        .expect("Blume Finder could not start");
}
